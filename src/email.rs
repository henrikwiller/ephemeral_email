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

/// Represents an email address with a name and domain.
/// The domain can be a predefined domain or a custom domain.
/// The name is the part before the `@` symbol.
/// The domain is the part after the `@` symbol.
/// # Example
/// ```
/// use ephemeral_email::{Domain, EmailAddress};
///
/// let email = EmailAddress::new("test", Domain::TenMinMailDe);
/// assert_eq!("test@10minmail.de", email.to_string());
///
/// let email: EmailAddress = "test@jaga.email".parse().unwrap();
/// assert_eq!("test", email.name);
/// assert_eq!(Domain::JagaEmail, email.domain);
///
/// let email: Result<EmailAddress, _> = "invalid-email".parse();
/// assert!(email.is_err());
/// ```
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_email_address_display() {
        assert_eq!(
            EmailAddress::new("test", Domain::TenMinMailDe).to_string(),
            "test@10minmail.de"
        );
        assert_eq!(
            EmailAddress::new("test", Domain::Custom("custom.com".into())).to_string(),
            "test@custom.com"
        );
    }

    #[test]
    fn test_email_address_from_str() {
        let email: EmailAddress = "test@10minmail.de".parse().unwrap();
        assert_eq!(email.name, "test");
        assert_eq!(email.domain, Domain::TenMinMailDe);

        let email: EmailAddress = "test@custom.com".parse().unwrap();
        assert_eq!(email.name, "test");
        assert_eq!(email.domain, Domain::Custom("custom.com".into()));
    }

    #[test]
    fn test_email_address_from_str_invalid() {
        let email: Result<EmailAddress, _> = "invalid-email".parse();
        assert!(email.is_err());
    }
}
