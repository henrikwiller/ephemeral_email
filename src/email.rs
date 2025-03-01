use std::fmt::{self, Display, Formatter};

use crate::domain::Domain;

#[derive(Debug)]
pub struct Message {
    pub from: Option<String>,
    pub subject: Option<String>,
    pub body: Option<String>,
}

#[derive(Debug)]
pub struct EmailAddress {
    pub name: String,
    pub domain: Domain,
}

impl EmailAddress {
    pub fn new(name: String, domain: Domain) -> Self {
        Self { name, domain }
    }
}

impl Display for EmailAddress {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}@{}", self.name, self.domain)
    }
}
