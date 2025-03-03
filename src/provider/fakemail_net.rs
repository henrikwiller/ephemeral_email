use std::{sync::Arc, vec};

use futures::lock::Mutex;
use rquest::Client;

use crate::{Domain, EmailAddress, InboxCreationError, Message, MessageFetcherError};

use super::{Inbox, Provider, ProviderType};

pub(crate) struct FakeMailNetProvider {}

pub(crate) struct FakeMailNetMessageFetcher {
    client: Client,
}

impl FakeMailNetProvider {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(serde::Deserialize)]
struct IndexResponse {
    email: String,
}

#[derive(serde::Deserialize)]
struct EmailListEntry {
    #[serde(rename = "predmet")]
    subject: String,
    #[serde(rename = "od")]
    from: String,
}

async fn get_csrf_token(client: &Client) -> Result<String, InboxCreationError> {
    let response = client
        .get("https://www.fakemail.net/")
        .send()
        .await?
        .text()
        .await?;
    let csrf_token = response
        .split_once("CSRF=\"")
        .ok_or(InboxCreationError::CreationError(
            "Failed to find CSRF token".to_string(),
        ))?
        .1
        .split_once("\"")
        .ok_or(InboxCreationError::CreationError(
            "Failed to extract CSRF token".to_string(),
        ))?
        .0;
    Ok(csrf_token.to_string())
}

#[async_trait::async_trait]
impl Provider for FakeMailNetProvider {
    async fn new_inbox(&mut self, name: &str, domain: Domain) -> Result<Inbox, InboxCreationError> {
        let email = EmailAddress::new(name, domain);
        let client = Client::builder().cookie_store(true).build()?;
        let csrf_token = get_csrf_token(&client).await?;
        let response = client
            .get("https://www.fakemail.net/index/index")
            .header("X-Requested-With", "XMLHttpRequest")
            .query(&[("csrf_token", csrf_token)])
            .send()
            .await?;
        if !response.status().is_success() {
            return Err(InboxCreationError::CreationError(
                response.text().await.unwrap_or_default(),
            ));
        }
        let check_response = client
            .post("https://www.fakemail.net/index/email-check")
            .header("X-Requested-With", "XMLHttpRequest")
            .form(&[("email", email.name.as_str()), ("format", "json")])
            .send()
            .await?
            .text()
            .await?;
        if check_response != "\"ok\"" {
            return Err(InboxCreationError::NameTaken(email.name));
        }

        let create_response = client
            .post("https://www.fakemail.net/index/new-email")
            .header("X-Requested-With", "XMLHttpRequest")
            .form(&[("emailInput", email.name.as_str()), ("format", "json")])
            .send()
            .await?
            .text()
            .await?;
        if create_response != "\"ok\"" {
            return Err(InboxCreationError::CreationError(create_response));
        }
        let csrf_token = get_csrf_token(&client).await?;
        let response = client
            .get("https://www.fakemail.net/index/index")
            .header("X-Requested-With", "XMLHttpRequest")
            .query(&[("csrf_token", csrf_token)])
            .send()
            .await?
            .text()
            .await?;
        let response = response.trim();
        let index_response: IndexResponse = serde_json::from_str(response.trim()).map_err(|e| {
            InboxCreationError::CreationError(format!("Failed to parse JSON: {}", e))
        })?;
        Ok(Inbox {
            message_fetcher: Arc::new(Mutex::new(FakeMailNetMessageFetcher { client })),
            email_address: index_response.email.parse()?,
        })
    }

    fn get_provider_type(&self) -> ProviderType {
        ProviderType::FakeMailNet
    }

    fn get_domains(&self) -> Vec<Domain> {
        vec![Domain::FileSavedOrg]
    }
}

#[async_trait::async_trait]
impl super::MessageFetcher for FakeMailNetMessageFetcher {
    async fn fetch_messages(&mut self) -> Result<Vec<Message>, MessageFetcherError> {
        let email_list_response = self
            .client
            .get("https://www.fakemail.net/index/refresh")
            .header("X-Requested-With", "XMLHttpRequest")
            .send()
            .await?
            .text()
            .await?;
        let email_list: Vec<EmailListEntry> = serde_json::from_str(email_list_response.trim())
            .map_err(|e| MessageFetcherError::FetchError(format!("Failed to parse JSON: {}", e)))?;

        let mut messages = Vec::new();
        for (i, email) in email_list.iter().rev().enumerate().skip(1) {
            let email_content = self
                .client
                .get(format!("https://www.fakemail.net/email/id/{}", i + 1))
                .header("X-Requested-With", "XMLHttpRequest")
                .send()
                .await?
                .text()
                .await?;
            messages.push(Message {
                from: email.from.clone(),
                subject: email.subject.clone(),
                body: email_content,
            });
        }
        Ok(messages)
    }
}
