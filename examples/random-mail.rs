use anyhow::Result;
use ephemeral_email::{provider::ProviderType, temp_mail::TempMail};

#[tokio::main]
async fn main() -> Result<()> {
    let mut inbox = TempMail::new()
        .provider_type(ProviderType::Muellmail)
        // .domain(Domain::RamenMailDe)
        .name("test")
        .create_inbox()
        .await?;
    let messages = inbox.get_messages().await?;
    println!(
        "Got {} messages for email {}",
        messages.len(),
        inbox.get_email_address()
    );
    for message in messages {
        println!("From: {}", message.from.unwrap());
    }
    Ok(())
}
