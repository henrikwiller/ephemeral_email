use crate::client::StatusCode;

use crate::provider::ProviderType;

/// Represents an error that can occur when creating a temporary email inbox.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum InboxCreationError {
    #[error("Request error: {0}")]
    RquestError(#[from] crate::client::Error),
    /// A generic error that occurred when creating an inbox.
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
    #[error("Rate limited, try again later")]
    RateLimited,
}

/// Represents an error that can occur when fetching a message from an inbox.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum MessageFetcherError {
    #[error("Request error: {0}")]
    RquestError(#[from] crate::client::Error),
    #[error("Invalid response status: {0}")]
    InvalidResponseStatus(StatusCode),
    /// A generic error that occurred when fetching a message.
    #[error("Failed to fetch message: {0}")]
    FetchError(String),
}

/// Represents an error that can occur when parsing an email address.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum EmailAddressError {
    #[error("Invalid email address: {0}")]
    InvalidEmailAddress(String),
}
