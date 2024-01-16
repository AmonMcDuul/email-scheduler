use chrono::Utc;

use crate::{models::message::Message, repository::database::Database};

pub fn send_scheduled_emails(database: &Database, messages: Vec<Message>) {
    for message in messages {
        if let Some(send_at) = message.send_at {
            let current_time = Utc::now();
            if send_at <= current_time && message.send != Some(true) {
                // todo implement email logic
                println!(
                    "Sending email to {} with message: {}",
                    message.email,
                    message.message_body.as_ref().map_or("", String::as_str)
                );
                if let Some(ref id) = message.id {
                    database.update_message_as_sent(id, message.clone());
                }
            } else {
                println!("i didn't send no mail yo!")
            }
        }
    }
}
