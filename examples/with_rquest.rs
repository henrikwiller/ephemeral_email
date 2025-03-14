use anyhow::Result;
use ephemeral_email::{ProviderType, TempMail};

#[tokio::main]
async fn main() -> Result<()> {
    let inbox = TempMail::new()
        .provider_type(ProviderType::Muellmail)
        .name("test")
        .create_inbox()
        .await?;
    println!("Created inbox with email: {}", inbox.get_email_address());
    let messages = inbox.get_messages().await?;
    println!("Got {} messages:", messages.len());
    for message in messages {
        println!("From: {}", message.from);
    }
    Ok(())
}
