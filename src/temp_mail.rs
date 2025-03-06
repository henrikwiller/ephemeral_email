use crate::{domain::Domain, error::InboxCreationError, provider::ProviderType, Inbox};

/// A builder for creating temporary email inboxes.
///
/// A builder for creating temporary email inboxes with customizable attributes.
/// You can specify the provider type, domain, and name for the inbox.
/// If any attribute is not specified, a random value will be chosen.
/// If a domain is specified, a compatible provider will be selected.
///
/// # Example
/// ```no_run
/// use ephemeral_email::{TempMail, Domain, ProviderType};
///
/// #[tokio::main]
/// async fn main() {
///     let inbox = TempMail::new()
///         .provider_type(ProviderType::TempMailLol)
///         .domain(Domain::TerribleCoffeeOrg)
///         .name("example")
///         .create_inbox()
///         .await
///         .unwrap();
/// }
/// ```
#[derive(Debug, Clone, Default)]
pub struct TempMail {
    provider_type: Option<ProviderType>,
    domain: Option<Domain>,
    name: Option<String>,
}

impl TempMail {
    /// Creates a new `TempMail` builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the provider type for the inbox.
    /// If a domain is specified, the provider must support that domain.
    /// If not specified, a random provider will be chosen, or if a domain is set, a provider supporting that domain will be selected.
    pub fn provider_type(mut self, provider_type: ProviderType) -> Self {
        self.provider_type = Some(provider_type);
        self
    }

    /// Specifies the domain for the inbox.
    /// If a provider type is already set, the domain must be supported by that provider.
    /// If no provider type is set, a provider supporting the domain will be chosen randomly.
    /// If no domain is specified, a random domain will be selected.
    pub fn domain(mut self, domain: Domain) -> Self {
        self.domain = Some(domain);
        self
    }

    /// Sets the name for the email address.
    ///
    /// Example: `example` will create an email address like `example@domain`.
    ///
    /// If not set, a random name will be chosen.
    /// Note that some providers allow reusing the same name, while others do not.
    /// If the provider does not allow reuse, an error may occur if the name is already taken.
    /// Providers may also impose restrictions on name length or characters.
    /// Some providers only allow setting the prefix of the email address, so verify the email address using the [`Inbox::get_email_address`] function.
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Creates a new inbox with the specified attributes.
    /// If no attributes are set, random values will be chosen.
    /// If a domain and provider is specified, they must be compatible.
    /// # Returns
    /// A new inbox, or an [`InboxCreationError`] error if the inbox could not be created.
    pub async fn create_inbox(self) -> Result<Inbox, InboxCreationError> {
        let provider_type = match (self.provider_type, &self.domain) {
            (Some(provider_type), None) => provider_type,
            (Some(provider_type), Some(_)) => provider_type,
            (None, None) => rand::random(),
            (None, Some(domain)) => {
                let provider_types = ProviderType::get_all_providers();
                *provider_types
                    .iter()
                    .find(|provider_type| {
                        provider_type.get_provider().get_domains().contains(domain)
                    })
                    .ok_or(InboxCreationError::NoProviderForDomain(domain.to_string()))?
            }
        };
        let mut provider = provider_type.get_provider();

        if let Some(ref domain) = self.domain {
            if !(provider.get_domains().contains(domain)
                || provider.support_custom_domains() && matches!(domain, Domain::Custom(_)))
            {
                return Err(InboxCreationError::InvalidDomainForProvider(
                    domain.to_string(),
                    provider.get_provider_type(),
                ));
            }
        }

        match (self.name, self.domain) {
            (Some(name), Some(domain)) => provider.new_inbox(&name, domain).await,
            (Some(name), None) => provider.new_random_inbox_from_name(&name).await,
            (None, Some(domain)) => provider.new_random_inbox_from_domain(domain).await,
            (None, None) => provider.new_random_inbox().await,
        }
    }
}
