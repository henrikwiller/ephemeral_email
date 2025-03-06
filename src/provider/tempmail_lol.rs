use std::sync::Arc;

use crate::client::Client;
use futures::lock::Mutex;
use serde_json::json;

use crate::{Domain, InboxCreationError, Message, MessageFetcherError};

use super::{Inbox, MessageFetcher, Provider, ProviderType};

pub(crate) struct TempMailLolProvider {}

pub(crate) struct TempMailLolMessageFetcher {
    client: Client,
    token: String,
}

impl TempMailLolProvider {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(serde::Deserialize)]
struct FetchResponse {
    emails: Vec<Email>,
}

#[derive(serde::Deserialize)]
#[allow(dead_code)]
struct Email {
    from: String,
    to: String,
    subject: String,
    body: String,
    date: u64,
}

impl From<Email> for Message {
    fn from(email: Email) -> Self {
        Self {
            from: email.from,
            subject: email.subject,
            body: email.body,
        }
    }
}

#[derive(serde::Deserialize)]
struct InboxResponse {
    address: String,
    token: String,
}

#[async_trait::async_trait]
impl Provider for TempMailLolProvider {
    async fn new_random_inbox_from_name(
        &mut self,
        name: &str,
    ) -> Result<Inbox, InboxCreationError> {
        let client = Client::builder().cookie_store(true).build()?;
        let response = client
            .post("https://api.tempmail.lol/v2/inbox/create")
            .header("ACCEPT", "application/json")
            .json(&json!({"prefix": name}))
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(InboxCreationError::CreationError(
                response.text().await.unwrap_or_default(),
            ));
        }

        let inbox: InboxResponse = response.json().await?;
        Ok(Inbox {
            message_fetcher: Arc::new(Mutex::new(TempMailLolMessageFetcher {
                client,
                token: inbox.token,
            })),
            email_address: inbox.address.parse()?,
        })
    }

    async fn new_inbox(&mut self, name: &str, domain: Domain) -> Result<Inbox, InboxCreationError> {
        let client = Client::builder().cookie_store(true).build()?;
        let response = client
            .post("https://api.tempmail.lol/v2/inbox/create")
            .header("ACCEPT", "application/json")
            .json(&json!({
                "prefix": name,
                "domain": domain.to_string(),
            }))
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(InboxCreationError::CreationError(
                response.text().await.unwrap_or_default(),
            ));
        }

        let inbox: InboxResponse = response.json().await?;
        Ok(Inbox {
            message_fetcher: Arc::new(Mutex::new(TempMailLolMessageFetcher {
                client,
                token: inbox.token,
            })),
            email_address: inbox.address.parse()?,
        })
    }

    fn get_domains(&self) -> Vec<Domain> {
        vec![
            Domain::TerribleCoffeeOrg,
            Domain::UnderseaGolfCom,
            Domain::JailBreakEverythingCom,
            Domain::Awesome47,
            Domain::ExpiredToasterOrg,
            Domain::UndeadBankCom,
        ]
    }
    fn get_random_domain(&self) -> Result<Domain, InboxCreationError> {
        Err(InboxCreationError::DomainNotSupported)
    }
    fn get_provider_type(&self) -> ProviderType {
        ProviderType::TempMailLol
    }
    fn support_custom_domains(&self) -> bool {
        true
    }
}

#[async_trait::async_trait]
impl MessageFetcher for TempMailLolMessageFetcher {
    async fn fetch_messages(&mut self) -> Result<Vec<Message>, MessageFetcherError> {
        let client = &self.client;
        let response = client
            .get("https://api.tempmail.lol/v2/inbox")
            .header("ACCEPT", "application/json")
            .query(&[("token", &self.token)])
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(MessageFetcherError::InvalidResponseStatus(
                response.status(),
            ));
        }

        let response: FetchResponse = response.json().await?;
        Ok(response.emails.into_iter().map(Into::into).collect())
    }
}
