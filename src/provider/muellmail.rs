use crate::domain::Domain;
use crate::email::EmailAddress;
use crate::error::{InboxCreationError, InboxError};
use rand::distr::{Alphanumeric, SampleString};
use rquest::{Client, Impersonate};

use super::{Inbox, Provider};
use rand::seq::IndexedRandom;

pub(crate) struct MuellmailProvider {}

pub(crate) struct MuellmailInbox {
    email: EmailAddress,
    client: Client,
}

#[derive(serde::Deserialize)]
struct CsrfResponse {
    #[serde(rename = "csrfToken")]
    csrf_token: String,
}

#[derive(serde::Deserialize)]
struct AnonResponse {
    url: String,
}

#[derive(serde::Deserialize)]
struct MessageQueryResponse {
    data: MessageQueryData,
}

#[derive(serde::Deserialize)]
struct MessageQueryData {
    emails: Vec<Email>,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
struct Email {
    id: String,
    subject: String,
    sender: String,
    sender_name: String,
    read: Option<String>,
    created_at: String,
    has_html: bool,
    has_text: bool,
    html: Option<String>,
    text: Option<String>,
    size_in_bytes: i64,
}

impl From<Email> for crate::email::Message {
    fn from(email: Email) -> Self {
        Self {
            from: Some(email.sender),
            subject: Some(email.subject),
            body: email.text,
        }
    }
}

impl MuellmailProvider {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait::async_trait]
impl Provider for MuellmailProvider {
    async fn new_inbox(
        &mut self,
        name: Option<&str>,
        domain: Option<Domain>,
    ) -> Result<Box<dyn Inbox>, InboxCreationError> {
        let domain = domain.unwrap_or_else(|| {
            self.get_domains()
                .choose(&mut rand::rng())
                .expect("No domains available")
                .clone()
        });
        let name = name
            .map(ToString::to_string)
            .unwrap_or_else(|| Alphanumeric.sample_string(&mut rand::rng(), 8));
        let email = EmailAddress::new(name, domain);

        let client = Client::builder()
            .cookie_store(true)
            .impersonate(Impersonate::Firefox135)
            .build()?;

        let csrf_token: CsrfResponse = client
            .get("https://muellmail.com/api/auth/csrf")
            .send()
            .await?
            .json()
            .await?;

        let anon: AnonResponse = client
            .post("https://muellmail.com/api/auth/callback/anon")
            .form(&[
                ("redirect", "false"),
                ("muellmail", &email.to_string()),
                ("csrfToken", &csrf_token.csrf_token),
                ("callbackUrl", "https://muellmail.com/en"),
                ("json", "true"),
            ])
            .send()
            .await?
            .json()
            .await?;

        if anon.url != "https://muellmail.com/en" {
            return Err(InboxCreationError::SetupError(format!(
                "Invalid response url {}",
                anon.url
            )));
        }

        Ok(Box::new(MuellmailInbox { email, client }))
    }

    fn get_domains(&self) -> Vec<Domain> {
        vec![
            Domain::TenMinMailDe,
            Domain::TenMinutenMailXyz,
            Domain::ExistiertNet,
            Domain::FliegenderFish,
            Domain::JagaEmail,
            Domain::MdzEmail,
            Domain::MuellMailCom,
            Domain::MuelleMailCom,
            Domain::MuellMonster,
            Domain::MuellIcu,
            Domain::MuellIo,
            Domain::MuellXyz,
            Domain::MagSpamNet,
            Domain::FukaruCom,
            Domain::OidaIcu,
            Domain::PapierkorbMe,
            Domain::SpamCare,
            Domain::TonneTo,
            Domain::UltraFyi,
            Domain::WegwerfEmailDe,
            Domain::DsgvoParty,
            Domain::KnickerbockerbanDe,
            Domain::LambsauceDe,
            Domain::RamenMailDe,
            Domain::Ji5De,
            Domain::Ji6De,
            Domain::Ji7De,
            Domain::SudernDe,
            Domain::HihiLol,
            Domain::KeinDate,
            Domain::HolioDay,
            Domain::CornHolioDay,
            Domain::BungHolioDay,
            Domain::StacysMom,
        ]
    }
}

#[async_trait::async_trait]
impl Inbox for MuellmailInbox {
    async fn get_messages(&mut self) -> Result<Vec<crate::email::Message>, InboxError> {
        let session_response = self
            .client
            .get("https://muellmail.com/api/auth/session")
            .send()
            .await?;
        if session_response.status() != 200 {
            return Err(InboxError::GetMessageError(format!(
                "Invalid response status {}",
                session_response.status()
            )));
        }

        let message_query_response: MessageQueryResponse = self
            .client
            .post("https://muellmail.com/graphql")
            .json(&serde_json::json!({
                "operationName": "MailQuery",
                "variables": {},
                "query": r#"
                    query MailQuery {
                        emails(orderBy: {createdAt: desc}) {
                            id
                            subject
                            sender
                            senderName
                            read
                            createdAt
                            hasHtml
                            hasText
                            html
                            text
                            sizeInBytes
                            attachments {
                                id
                                emailId
                                fileName
                                headersHashed
                                contentType
                                sizeInBytes
                                createdAt
                                updatedAt
                                __typename
                            }
                            __typename
                        }
                    }
                "#
            }))
            .send()
            .await?
            .json()
            .await?;

        Ok(message_query_response
            .data
            .emails
            .into_iter()
            .map(Into::into)
            .collect())
    }

    fn get_email_address(&self) -> String {
        self.email.to_string()
    }
}
