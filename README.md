# Ephemeral Email

`ephemeral_email` is a Rust library for creating and managing temporary email addresses effortlessly. It allows developers to generate disposable email addresses, fetch messages from temporary inboxes, and supports multiple email providers. This is particularly useful for testing, avoiding spam, or any scenario where a temporary email address is needed. With support for 42 domains and a straightforward API, `ephemeral_email` simplifies handling temporary emails in your Rust applications.

## Features

- Create temporary email addresses
- Fetch messages from temporary inboxes
- Support for multiple email providers
- 42 domains supported

## Email Providers

You can use the following email providers for temporary email addresses:

- [Mail.tm](https://mail.tm)
- [Muellmail.com](https://muellmail.com)
- [TempMail.lol](https://tempmail.lol)
- [FakeMail.net](https://fakemail.net)

## Disclaimer

This library is not affiliated with, endorsed by, or associated with the email providers listed above. Use this library at your own risk. Users are responsible for ensuring their use of the email providers' services complies with the respective terms of service and any applicable laws and regulations. The author assumes no responsibility for any misuse or legal issues that may arise from using this library.

## Usage

`ephemeral_email` uses [rquest] to simulate browser behavior, making it incompatible with crates that depend on `openssl-sys`, such as [reqwest]. Ensure you use [rquest] instead of [reqwest]. For any build issues, consult the [rquest documentation](https://github.com/0x676e67/rquest#building).

[rquest]: https://github.com/0x676e67/rquest
[reqwest]: https://github.com/seanmonstar/reqwest

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
        .await
        .unwrap();
    println!("Created inbox with email: {}", inbox.get_email_address());
    let messages = inbox.get_messages().await.unwrap();
    println!("Got {} messages:", messages.len());
    for message in messages {
        println!("From: {}", message.from);
    }
}
```

## Related Crates

Check out these related crates:

- [tempmail-lol](https://github.com/Morb0/tempmail-lol)
- [tmail](https://github.com/atifyushri/tmail)

## Contact

If you are a representative of an email provider listed in this library and would like to request removal or have any concerns, please contact us at [ephemeral_email@hwiller.com](mailto:ephemeral_email@hwiller.com).
