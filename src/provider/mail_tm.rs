use std::sync::Arc;

use crate::client::Client;
use futures::lock::Mutex;
use serde_json::json;

use crate::{
    domain::Domain, email::EmailAddress, error::InboxCreationError, Message, MessageFetcherError,
};

use super::{Inbox, MessageFetcher, Provider};

pub(crate) struct MailTmProvider {}

pub(crate) struct MailTmMessageFetcher {
    client: Client,
    token: String,
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

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct DomainResponse {
    domain: String,
    is_private: bool,
    is_active: bool,
}

impl From<Email> for crate::email::Message {
    fn from(email: Email) -> Self {
        Self {
            from: email.from.address,
            subject: email.subject,
            body: email.text,
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

    Err(InboxCreationError::CreationError(format!(
        "Failed to login into: {}",
        email
    )))
}

#[async_trait::async_trait]
impl Provider for MailTmProvider {
    async fn new_random_inbox_from_name(
        &mut self,
        name: &str,
    ) -> Result<Inbox, InboxCreationError> {
        let client = Client::builder().cookie_store(true).build()?;

        let domain_response: Vec<DomainResponse> = client
            .get("https://api.mail.tm/domains")
            .header("ACCEPT", "application/json")
            .send()
            .await?
            .json()
            .await?;
        let Some(domain) = domain_response
            .iter()
            .filter(|d| !d.is_private && d.is_active)
            .next()
        else {
            return Err(InboxCreationError::CreationError(
                "No active domain found".to_string(),
            ));
        };
        let domain = Domain::from(domain.domain.as_str());

        let email = EmailAddress::new(name, domain);
        if let Ok(token) = try_login(&client, &email).await {
            return Ok(Inbox {
                message_fetcher: Arc::new(Mutex::new(MailTmMessageFetcher { client, token })),
                email_address: email,
            });
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
            if login_response.status() == 429 {
                return Err(InboxCreationError::RateLimited);
            }
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

            return Err(InboxCreationError::CreationError(format!(
                "Failed to create inbox: {}",
                violation
                    .violations
                    .first()
                    .map(|v| v.message.as_str())
                    .unwrap_or("Unknown error")
            )));
        }

        let token = try_login(&client, &email).await?;
        Ok(Inbox {
            message_fetcher: Arc::new(Mutex::new(MailTmMessageFetcher { client, token })),
            email_address: email,
        })
    }

    fn get_provider_type(&self) -> crate::provider::ProviderType {
        crate::provider::ProviderType::MailTm
    }

    fn get_domains(&self) -> Vec<Domain> {
        vec![]
    }
}

#[async_trait::async_trait]
impl MessageFetcher for MailTmMessageFetcher {
    async fn fetch_messages(&mut self) -> Result<Vec<Message>, MessageFetcherError> {
        let email_list: Vec<EmailListEntry> = self
            .client
            .get("https://api.mail.tm/messages")
            .bearer_auth(&self.token)
            .header("ACCEPT", "application/json")
            .send()
            .await?
            .json()
            .await?;

        let mut messages = Vec::new();
        for email in email_list {
            let email: Email = self
                .client
                .get(format!("https://api.mail.tm/messages/{}", email.id))
                .bearer_auth(&self.token)
                .header("ACCEPT", "application/json")
                .send()
                .await?
                .json()
                .await?;
            messages.push(email.into());
        }

        Ok(messages)
    }
}
