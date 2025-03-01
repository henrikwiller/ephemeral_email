use rand::{
    distr::{Alphanumeric, SampleString},
    seq::IndexedRandom,
};
use rquest::{header::HeaderValue, Client};
use serde_json::json;

use crate::{domain::Domain, email::EmailAddress, error::InboxCreationError};

use super::{Inbox, Provider};

pub(crate) struct MailTmProvider {}

pub(crate) struct MailTmInbox {
    email: EmailAddress,
    client: Client,
}

#[derive(serde::Deserialize)]
struct EmailListEntry {
    id: String,
}

#[derive(serde::Deserialize)]
#[allow(dead_code)]
struct FromHeader {
    address: String,
    name: String,
}

#[derive(serde::Deserialize)]
struct Email {
    from: FromHeader,
    subject: String,
    text: String,
}

#[derive(serde::Deserialize)]
struct Violation {
    status: u32,
    violations: Vec<ViolationEntry>,
}

#[derive(serde::Deserialize)]
struct ViolationEntry {
    #[serde(rename = "propertyPath")]
    property_path: String,
    message: String,
}

#[derive(serde::Deserialize)]
struct LoginResponse {
    token: String,
}

impl From<Email> for crate::email::Message {
    fn from(email: Email) -> Self {
        Self {
            from: Some(email.from.address),
            subject: Some(email.subject),
            body: Some(email.text),
        }
    }
}

impl MailTmProvider {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

async fn try_login(client: &Client, email: &EmailAddress) -> Result<String, InboxCreationError> {
    let login_response = client
        .post("https://api.mail.tm/token")
        .header("ACCEPT", "application/json")
        .json(&json!({
            "address": email.to_string(),
            "password": "ephemeral_email"
        }))
        .send()
        .await?;

    if login_response.status().is_success() {
        let login_response: LoginResponse = login_response.json().await?;
        return Ok(login_response.token);
    }

    Err(InboxCreationError::SetupError(format!(
        "Failed to login into: {}",
        email
    )))
}

#[async_trait::async_trait]
impl Provider for MailTmProvider {
    async fn new_inbox(
        &mut self,
        name: Option<&str>,
        domain: Option<Domain>,
    ) -> Result<Box<dyn Inbox>, InboxCreationError> {
        let domain = domain.unwrap_or_else(|| {
            self.get_domains()
                .choose(&mut rand::rng())
                .expect("No domains available")
                .clone()
        });
        let name = name
            .map(ToString::to_string)
            .unwrap_or_else(|| Alphanumeric.sample_string(&mut rand::rng(), 8));
        let email = EmailAddress::new(name, domain);

        let mut client = Client::builder().cookie_store(true).build()?;

        if let Ok(token) = try_login(&client, &email).await {
            client.as_mut().headers().append(
                "Authorization",
                HeaderValue::from_str(&format!("Bearer {}", token)).unwrap(),
            );
            return Ok(Box::new(MailTmInbox { email, client }));
        }

        let login_response = client
            .post("https://api.mail.tm/accounts")
            .header("ACCEPT", "application/json")
            .json(&json!({
                "address": email.to_string(),
                "password": "ephemeral_email"
            }))
            .send()
            .await?;
        if !login_response.status().is_success() {
            let violation: Violation = login_response.json().await?;
            if violation.status == 422 && violation.violations.len() == 1 {
                let violation = &violation.violations[0];
                if violation.property_path == "address"
                    && violation.message.ends_with("already used.")
                {
                    return Err(InboxCreationError::NameTaken(email.to_string()));
                }
                if violation.property_path == "address"
                    && violation.message.ends_with("is not valid.")
                {
                    return Err(InboxCreationError::InvalidName(email.to_string()));
                }
            }

            return Err(InboxCreationError::SetupError(format!(
                "Failed to create inbox: {}",
                violation
                    .violations
                    .first()
                    .map(|v| v.message.as_str())
                    .unwrap_or("Unknown error")
            )));
        }

        let token = try_login(&client, &email).await?;
        client.as_mut().headers().append(
            "Authorization",
            HeaderValue::from_str(&format!("Bearer {}", token)).unwrap(),
        );
        Ok(Box::new(MailTmInbox { email, client }))
    }

    fn get_domains(&self) -> Vec<Domain> {
        vec![Domain::EdnyNet]
    }
}

#[async_trait::async_trait]
impl Inbox for MailTmInbox {
    async fn get_messages(
        &mut self,
    ) -> Result<Vec<crate::email::Message>, crate::error::InboxError> {
        let email_list: Vec<EmailListEntry> = self
            .client
            .get("https://api.mail.tm/messages")
            .header("ACCEPT", "application/json")
            .send()
            .await?
            .json()
            .await?;

        let mut messages = Vec::new();
        for email in email_list {
            let email: Email = self
                .client
                .get(&format!("https://api.mail.tm/messages/{}", email.id))
                .header("ACCEPT", "application/json")
                .send()
                .await?
                .json()
                .await?;
            messages.push(email.into());
        }

        Ok(messages)
    }

    fn get_email_address(&self) -> String {
        self.email.to_string()
    }
}
