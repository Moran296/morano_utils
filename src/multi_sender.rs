use crate::traits::MoranoMessage;
use anyhow::Result;
use frankenstein::{Api, SendMessageParams};
use std::collections::VecDeque;

pub struct MoranoMultiSender {
    messages: VecDeque<Box<dyn MoranoMessage>>,
}

impl MoranoMultiSender {
    pub fn new() -> Self {
        Self {
            messages: VecDeque::new(),
        }
    }

    pub fn from(vector: Vec<Box<dyn MoranoMessage>>) -> Self {
        Self {
            messages: VecDeque::from(vector),
        }
    }

    pub fn with(mut self, message: Box<dyn MoranoMessage>) -> Self {
        self.messages.push_back(message);
        self
    }

    pub fn len(&self) -> usize {
        self.messages.len()
    }
}

impl MoranoMessage for MoranoMultiSender {
    fn to_msg_params(&self) -> SendMessageParams {
        if let Some(message) = self.messages.back() {
            message.to_msg_params()
        } else {
            panic!("No messages to send");
        }
    }

    fn send_to_chat(&self, api: &Api) -> Result<()> {
        for message in self.messages.iter() {
            message.send_to_chat(api)?;
        }

        Ok(())
    }
}
