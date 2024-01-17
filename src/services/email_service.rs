use chrono::Utc;
use lettre::message::Mailbox;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message as LettreMessage, SmtpTransport, Transport};
use std::fs;

use crate::{models::config, models::message::Message, repository::database::Database};

pub fn send_scheduled_emails(database: &Database, messages: Vec<Message>) {
    for message in messages {
        if let Some(send_at) = message.send_at {
            let current_time = Utc::now();
            if send_at <= current_time && message.send != Some(true) {
                println!(
                    "Sending email to {} with message: {}",
                    message.email,
                    message.message_body.as_ref().map_or("", String::as_str)
                );
                let subject = "E-mail scheduler";

                if let Err(err) = send_email(
                    &message.email,
                    subject,
                    message.message_body.as_ref().map_or("", String::as_str),
                ) {
                    println!("Failed to send email: {:?}", err);
                } else {
                    database.update_message_as_sent(message.clone());
                }
            } else {
                println!("I didn't send no mail yo!")
            }
        }
    }
}

pub fn send_email(to: &str, subject: &str, body: &str) -> Result<(), Box<dyn std::error::Error>> {
    let config_str = fs::read_to_string("config.toml").expect("Error reading config file");
    let config: config::AppConfig = toml::from_str(&config_str).expect("Error parsing config file");

    let username = &config.credentials.username;
    let password = &config.credentials.password;

    let from_address: Mailbox = username.parse()?;
    let to_address: Mailbox = to.parse()?;

    let email = LettreMessage::builder()
        .from(from_address)
        .to(to_address)
        .subject(subject)
        .body(body.to_owned())?;

    let creds = Credentials::new(username.to_string(), password.to_string());
    let mailer = SmtpTransport::starttls_relay("smtp.gmail.com")?
        .credentials(creds)
        .build();

    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully"),
        Err(e) => println!("Failed to send email: {:?}", e),
    }
    Ok(())
}
