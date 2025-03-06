#[cfg(feature = "use-rquest")]
pub use rquest::{Client, Error, StatusCode};

#[cfg(not(feature = "use-rquest"))]
pub use reqwest::{Client, Error, StatusCode};
