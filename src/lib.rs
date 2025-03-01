mod domain;
mod email;
mod error;
mod provider;
mod temp_mail;

pub use domain::Domain;
pub use email::{EmailAddress, Message};
pub use error::{BuilderError, InboxCreationError, InboxError};
pub use provider::{Inbox, ProviderType};
pub use temp_mail::TempMail;
