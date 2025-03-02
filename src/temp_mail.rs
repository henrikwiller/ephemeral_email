use crate::{
    domain::Domain, error::InboxCreationError, provider::ProviderType, EmailAddress, Inbox,
};

pub struct TempMail {
    provider_type: Option<ProviderType>,
    domain: Option<Domain>,
    name: Option<String>,
}

impl TempMail {
    pub fn new() -> Self {
        Self {
            provider_type: None,
            domain: None,
            name: None,
        }
    }

    pub fn provider_type(mut self, provider_type: ProviderType) -> Self {
        self.provider_type = Some(provider_type);
        self
    }

    pub fn domain(mut self, domain: Domain) -> Self {
        self.domain = Some(domain);
        self
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub async fn create_inbox(self) -> Result<Inbox, InboxCreationError> {
        let provider_type = match (self.provider_type, self.domain) {
            (Some(provider_type), None) => provider_type,
            (Some(provider_type), Some(_)) => provider_type,
            (None, None) => rand::random(),
            (None, Some(_)) => {
                let provider_types = ProviderType::get_all_providers();
                *provider_types
                    .iter()
                    .find(|provider_type| {
                        provider_type
                            .get_provider()
                            .get_domains()
                            .contains(&self.domain.unwrap())
                    })
                    .ok_or(InboxCreationError::NoProviderForDomain(
                        self.domain.unwrap().to_string(),
                    ))?
            }
        };
        let mut provider = provider_type.get_provider();

        match (self.name, self.domain) {
            (Some(name), Some(domain)) => {
                provider
                    .new_inbox_from_email(EmailAddress::new(name, domain))
                    .await
            }
            (Some(name), None) => provider.new_random_inbox_from_name(&name).await,
            (None, Some(domain)) => provider.new_random_inbox_from_domain(domain).await,
            (None, None) => provider.new_random_inbox().await,
        }
    }
}
