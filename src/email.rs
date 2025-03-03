use std::{
    fmt::{self, Display, Formatter},
    str::FromStr,
};

use crate::{domain::Domain, error::EmailAddressError};

/// Represents an email message with a sender, subject, and body.
/// The body can be plain text or HTML.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Message {
    /// The sender of the email message.
    pub from: String,
    /// The subject of the email message.
    pub subject: String,
    /// The main content of the email, which can be plain text or HTML.
    pub body: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EmailAddress {
    pub name: String,
    pub domain: Domain,
}

impl EmailAddress {
    pub fn new(name: impl Into<String>, domain: Domain) -> Self {
        Self {
            name: name.into(),
            domain,
        }
    }
}

impl Display for EmailAddress {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}@{}", self.name, self.domain)
    }
}

impl FromStr for EmailAddress {
    type Err = EmailAddressError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, domain) = s
            .split_once('@')
            .ok_or(EmailAddressError::InvalidEmailAddress(s.into()))?;

        Ok(Self {
            name: name.into(),
            domain: domain.into(),
        })
    }
}
