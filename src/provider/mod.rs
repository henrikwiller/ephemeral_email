use std::fmt::{self, Display, Formatter};

use muellmail::MuellmailProvider;
use rand::distr::{Alphanumeric, Distribution, SampleString, StandardUniform};
use rand::seq::IndexedRandom;

use crate::domain::Domain;
use crate::error::InboxCreationError;
use crate::{EmailAddress, InboxError, Message};

mod mail_tm;
mod muellmail;

#[async_trait::async_trait]
pub(crate) trait Provider: Send + Sync {
    async fn new_random_inbox(&mut self) -> Result<Inbox, InboxCreationError> {
        let domain = self
            .get_domains()
            .choose(&mut rand::rng())
            .expect("No domains available")
            .clone();

        self.new_random_inbox_from_domain(domain).await
    }

    async fn new_random_inbox_from_domain(
        &mut self,
        domain: Domain,
    ) -> Result<Inbox, InboxCreationError> {
        if !self.get_domains().contains(&domain) {
            return Err(InboxCreationError::InvalidDomainForProvider(
                domain.to_string(),
                self.get_provider_type(),
            ));
        }
        let name = Alphanumeric.sample_string(&mut rand::rng(), 8);
        self.new_inbox_from_email(EmailAddress::new(name, domain))
            .await
    }

    async fn new_random_inbox_from_name(
        &mut self,
        name: &str,
    ) -> Result<Inbox, InboxCreationError> {
        let domain = self
            .get_domains()
            .choose(&mut rand::rng())
            .expect("No domains available")
            .clone();

        self.new_inbox_from_email(EmailAddress::new(name.into(), domain))
            .await
    }

    async fn new_inbox_from_email(
        &mut self,
        _email: EmailAddress,
    ) -> Result<Inbox, InboxCreationError> {
        Err(InboxCreationError::ProviderNotImplemented)
    }

    fn get_domains(&self) -> Vec<Domain>;
    fn get_provider_type(&self) -> ProviderType;
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
