use anyhow::{anyhow, Result};
use frankenstein::{Api, CallbackQuery, Message, SendMessageParams, TelegramApi};

/// Trait for all sendable messages to chat
pub trait MoranoMessage {
    fn to_msg_params(&self) -> SendMessageParams;
    fn send_to_chat(&self, api: &Api) -> Result<()>;
}

impl MoranoMessage for SendMessageParams {
    fn to_msg_params(&self) -> SendMessageParams {
        self.clone()
    }
    fn send_to_chat(&self, api: &Api) -> Result<()> {
        if let Err(e) = api.send_message(&self) {
            Err(anyhow!("{:?}", e))
        } else {
            Ok(())
        }
    }
}

pub trait MoranoDialouge {
    fn on_message(&mut self, message: &Message) -> Option<Box<dyn MoranoMessage>>;
    fn on_callback_query(&mut self, query: &CallbackQuery) -> Option<Box<dyn MoranoMessage>>;
}
