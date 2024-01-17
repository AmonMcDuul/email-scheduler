use chrono::prelude::*;
use std::fmt::Error;
use std::sync::{Arc, Mutex};

use crate::models::message::Message;

pub struct Database {
    pub messages: Arc<Mutex<Vec<Message>>>,
}

impl Database {
    pub fn new() -> Self {
        let messages = Arc::new(Mutex::new(vec![]));
        Database { messages }
    }

    pub fn get_messages(&self) -> Vec<Message> {
        let messages = self.messages.lock().unwrap();
        messages.clone()
    }

    pub fn get_message_by_id(&self, id: &str) -> Option<Message> {
        let messages = self.messages.lock().unwrap();
        messages
            .iter()
            .find(|message| message.id == Some(id.to_string()))
            .cloned()
    }

    pub fn create_message(&self, message: Message) -> Result<Message, Error> {
        let id = uuid::Uuid::new_v4().to_string();
        let mut messages = self.messages.lock().unwrap();
        let created_at = Utc::now();
        let send = false;
        let message = Message {
            id: Some(id),
            created_at: Some(created_at),
            send: Some(send),
            ..message
        };
        messages.push(message.clone());
        Ok(message)
    }

    pub fn update_message_by_id(&self, id: &str, message: Message) -> Option<Message> {
        let mut messages = self.messages.lock().unwrap();
        let message = Message {
            id: Some(id.to_string()),
            ..message
        };
        let index = messages
            .iter()
            .position(|message: &Message| message.id == Some(id.to_string()))?;
        messages[index] = message.clone();
        Some(message)
    }

    pub fn delete_message_by_id(&self, id: &str) -> Option<Message> {
        let mut messages = self.messages.lock().unwrap();
        let index = messages
            .iter()
            .position(|message| message.id == Some(id.to_string()))?;
        Some(messages.remove(index))
    }

    pub fn update_message_as_sent(&self, message: Message) -> Option<Message> {
        let mut messages = self.messages.lock().unwrap();
        let message = Message {
            id: Some(message.id?),
            send: Some(true),
            ..message
        };
        let index = messages
            .iter()
            .position(|msg: &Message| msg.id == message.id)?;
        messages[index] = message.clone();
        Some(message)
    }
}
