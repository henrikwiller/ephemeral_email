use std::fmt::{self, Display, Formatter};
use std::sync::Arc;

use futures::lock::Mutex;
use muellmail::MuellmailProvider;
use rand::distr::{Alphanumeric, Distribution, SampleString, StandardUniform};
use rand::seq::IndexedRandom;

use crate::domain::Domain;
use crate::error::InboxCreationError;
use crate::{EmailAddress, Message, MessageFetcherError};

mod fakemail_net;
mod mail_tm;
mod muellmail;
mod tempmail_lol;

#[async_trait::async_trait]
pub(crate) trait Provider: Send + Sync {
    async fn new_random_inbox(&mut self) -> Result<Inbox, InboxCreationError> {
        if let Ok(domain) = self.get_random_domain() {
            return self.new_random_inbox_from_domain(domain).await;
        }
        self.new_random_inbox_from_name(&self.get_random_name())
            .await
    }

    async fn new_random_inbox_from_domain(
        &mut self,
        domain: Domain,
    ) -> Result<Inbox, InboxCreationError> {
        self.new_inbox(&self.get_random_name(), domain).await
    }

    async fn new_random_inbox_from_name(
        &mut self,
        name: &str,
    ) -> Result<Inbox, InboxCreationError> {
        self.new_inbox(name, self.get_random_domain()?).await
    }

    async fn new_inbox(
        &mut self,
        _name: &str,
        _domain: Domain,
    ) -> Result<Inbox, InboxCreationError> {
        Err(InboxCreationError::ProviderNotImplemented)
    }

    fn get_random_name(&self) -> String {
        Alphanumeric.sample_string(&mut rand::rng(), 8)
    }
    fn get_random_domain(&self) -> Result<Domain, InboxCreationError> {
        self.get_domains()
            .choose(&mut rand::rng())
            .ok_or(InboxCreationError::DomainNotSupported)
            .cloned()
    }
    fn get_domains(&self) -> Vec<Domain> {
        vec![]
    }
    fn get_provider_type(&self) -> ProviderType;
    fn support_custom_domains(&self) -> bool {
        false
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ProviderType {
    FakeMailNet,
    MailTm,
    Muellmail,
    TempMailLol,
}

impl ProviderType {
    pub(crate) fn get_provider(&self) -> Box<dyn Provider> {
        match self {
            ProviderType::FakeMailNet => Box::new(fakemail_net::FakeMailNetProvider::new()),
            ProviderType::MailTm => Box::new(mail_tm::MailTmProvider::new()),
            ProviderType::Muellmail => Box::new(MuellmailProvider::new()),
            ProviderType::TempMailLol => Box::new(tempmail_lol::TempMailLolProvider::new()),
        }
    }

    pub(crate) fn get_all_providers() -> Vec<ProviderType> {
        vec![
            ProviderType::FakeMailNet,
            ProviderType::MailTm,
            ProviderType::Muellmail,
            ProviderType::TempMailLol,
        ]
    }
}

impl Display for ProviderType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            ProviderType::FakeMailNet => write!(f, "FakeMail.net"),
            ProviderType::MailTm => write!(f, "Mail.tm"),
            ProviderType::Muellmail => write!(f, "Muellmail"),
            ProviderType::TempMailLol => write!(f, "TempMail.lol"),
        }
    }
}

impl Distribution<ProviderType> for StandardUniform {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> ProviderType {
        let provider_types = ProviderType::get_all_providers();
        let index = rng.random_range(0..provider_types.len());
        provider_types[index]
    }
}

#[async_trait::async_trait]
pub trait MessageFetcher: Send + Sync {
    async fn fetch_messages(&mut self) -> Result<Vec<Message>, MessageFetcherError>;
}

pub struct Inbox {
    message_fetcher: Arc<Mutex<dyn MessageFetcher>>,
    email_address: EmailAddress,
}

impl Inbox {
    pub fn get_email_address(&self) -> &EmailAddress {
        &self.email_address
    }

    pub async fn get_messages(&self) -> Result<Vec<Message>, MessageFetcherError> {
        self.message_fetcher.lock().await.fetch_messages().await
    }
}
