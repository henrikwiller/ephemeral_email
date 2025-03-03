# Ephemeral Email

Ephemeral Email is a Rust library for creating and managing temporary email addresses.

## Features
- Create temporary email addresses
- Fetch messages from temporary inboxes
- Supports multiple email providers

## Email Providers
Here are some email providers you can use for temporary email addresses:
- [Mail.tm](https://mail.tm)
- [Muellmail.com](https://muellmail.com)
- [TempMail.lol](https://tempmail.lol)
- [FakeMail.net](https://fakemail.net)

## Usage
Add the following to your `Cargo.toml`:
```toml
[dependencies]
ephemeral_email = "0.1"
```

Example usage:
```rust
use ephemeral_email::{ProviderType, TempMail};

#[tokio::main]
async fn main() {
    let inbox = TempMail::new()
        .provider_type(ProviderType::Muellmail)
        .name("test")
        .create_inbox()
        .await?;
    println!("Created inbox with email: {}", inbox.get_email_address());
    let messages = inbox.get_messages().await.unwrap();
    println!("Got {} messages:", messages.len());
    for message in messages {
        println!("From: {}", message.from);
    }
    Ok(())
}
```

## Other Crates
Check out these related crates:
- [tempmail-lol](https://github.com/Morb0/tempmail-lol)
- [tmail](https://github.com/atifyushri/tmail)

## License
This project is licensed under the MIT License.
