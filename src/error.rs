use rquest::StatusCode;

use crate::provider::ProviderType;

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum InboxCreationError {
    #[error("Request error: {0}")]
    RquestError(#[from] rquest::Error),
    #[error("Cannot create inbox: {0}")]
    CreationError(String),
    #[error("Provider not implemented")]
    ProviderNotImplemented,
    #[error("Provider does not support specifying a domain")]
    DomainNotSupported,
    #[error("Could not find a provider for {0}")]
    NoProviderForDomain(String),
    #[error("The domain {0} is not valid for provider {1}")]
    InvalidDomainForProvider(String, ProviderType),
    #[error("Name is already taken by someone else: {0}")]
    NameTaken(String),
    #[error("Name is invalid: {0}")]
    InvalidName(String),
    #[error("An invalid email address was returned: {0}")]
    InvalidEmailAddress(#[from] EmailAddressError),
}

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum MessageFetcherError {
    #[error("Request error: {0}")]
    RquestError(#[from] rquest::Error),
    #[error("Invalid response status: {0}")]
    InvalidResponseStatus(StatusCode),
}

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum EmailAddressError {
    #[error("Invalid email address: {0}")]
    InvalidEmailAddress(String),
}
