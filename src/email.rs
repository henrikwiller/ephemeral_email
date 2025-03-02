use std::{
    fmt::{self, Display, Formatter},
    str::FromStr,
};

use crate::{domain::Domain, error::EmailAddressError};

#[derive(Debug)]
pub struct Message {
    pub from: Option<String>,
    pub subject: Option<String>,
    pub body: Option<String>,
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
