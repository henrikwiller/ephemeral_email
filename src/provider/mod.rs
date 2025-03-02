use std::fmt::{self, Display, Formatter};

use muellmail::MuellmailProvider;
use rand::distr::{Distribution, StandardUniform};

use crate::domain::Domain;
use crate::error::InboxCreationError;
use crate::{EmailAddress, InboxError, Message};

mod mail_tm;
mod muellmail;

#[async_trait::async_trait]
pub(crate) trait Provider {
    async fn new_inbox(
        &mut self,
        name: Option<&str>,
        domain: Option<Domain>,
    ) -> Result<Inbox, InboxCreationError>;

    fn get_domains(&self) -> Vec<Domain>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ProviderType {
    MailTm,
    Muellmail,
}

impl ProviderType {
    pub(crate) fn get_provider(&self) -> Box<dyn Provider> {
        match self {
            ProviderType::MailTm => Box::new(mail_tm::MailTmProvider::new()),
            ProviderType::Muellmail => Box::new(MuellmailProvider::new()),
        }
    }

    pub(crate) fn get_all_providers() -> Vec<ProviderType> {
        vec![ProviderType::MailTm, ProviderType::Muellmail]
    }
}

impl Display for ProviderType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            ProviderType::MailTm => write!(f, "Mail.tm"),
            ProviderType::Muellmail => write!(f, "Muellmail"),
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
    async fn get_messages(&mut self) -> Result<Vec<Message>, InboxError>;
}

pub struct Inbox {
    message_fetcher: Box<dyn MessageFetcher>,
    email_address: EmailAddress,
}

impl Inbox {
    pub fn get_email_address(&self) -> &EmailAddress {
        &self.email_address
    }

    pub async fn get_messages(&mut self) -> Result<Vec<Message>, InboxError> {
        self.message_fetcher.get_messages().await
    }
}
