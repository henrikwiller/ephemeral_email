use std::fmt::{self, Display, Formatter};

use muellmail::MuellmailProvider;
use rand::distr::{Distribution, StandardUniform};

use crate::domain::Domain;
use crate::email::Message;
use crate::error::{InboxCreationError, InboxError};

mod muellmail;

#[async_trait::async_trait]
pub trait Inbox {
    async fn get_messages(&mut self) -> Result<Vec<Message>, InboxError>;
    fn get_email_address(&self) -> String;
}

#[async_trait::async_trait]
pub(crate) trait Provider {
    async fn new_inbox(
        &mut self,
        name: Option<&str>,
        domain: Option<Domain>,
    ) -> Result<Box<dyn Inbox>, InboxCreationError>;

    fn get_domains(&self) -> Vec<Domain>;
}

#[derive(Debug, Clone, Copy)]
pub enum ProviderType {
    Muellmail,
}

impl ProviderType {
    pub(crate) fn get_provider(&self) -> Box<dyn Provider> {
        match self {
            ProviderType::Muellmail => Box::new(MuellmailProvider::new()),
        }
    }

    pub(crate) fn get_all_providers() -> Vec<ProviderType> {
        vec![ProviderType::Muellmail]
    }
}

impl Distribution<ProviderType> for StandardUniform {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> ProviderType {
        let provider_types = ProviderType::get_all_providers();
        let index = rng.random_range(0..provider_types.len());
        provider_types[index]
    }
}

impl Display for ProviderType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            ProviderType::Muellmail => write!(f, "Muellmail"),
        }
    }
}
