use crate::provider::ProviderType;

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum InboxCreationError {
    #[error("Request error: {0}")]
    RquestError(#[from] rquest::Error),
    #[error("Cannot setup email: {0}")]
    SetupError(String),
    #[error("Could not find a provider for {0}")]
    NoProviderForDomain(String),
    #[error("The domain {0} is not valid for provider {1}")]
    InvalidDomainForProvider(String, ProviderType),
    #[error("Name is already taken by someone else: {0}")]
    NameTaken(String),
    #[error("Name is invalid: {0}")]
    InvalidName(String),
    #[error("Provider not implemented")]
    ProviderNotImplemented,
}

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum InboxError {
    #[error("Request error: {0}")]
    RquestError(#[from] rquest::Error),
    #[error("Cannot get messages: {0}")]
    GetMessageError(String),
}

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum BuilderError {
    #[error("Cannot create inbox: {0}")]
    InboxCreationError(#[from] InboxCreationError),

    #[error("Cannot specify name for provider {0}")]
    NameSpecified(ProviderType),
}
