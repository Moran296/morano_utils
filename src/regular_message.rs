use crate::traits::MoranoMessage;
use anyhow::{anyhow, Result};
use frankenstein::{Api, ChatId, ForceReply, ReplyMarkup, SendMessageParams, TelegramApi};

pub struct RegularMessage {
    text: String,
    id: u64,
    pub reply_to_message_id: Option<i32>,
    pub forced_to_reply: bool,
}

impl RegularMessage {
    pub fn new(id: u64, text: &str) -> RegularMessage {
        RegularMessage {
            text: text.to_string(),
            id,
            reply_to_message_id: None,
            forced_to_reply: false,
        }
    }

    pub fn new_reply(id: u64, text: String, reply_to_message_id: Option<i32>) -> RegularMessage {
        RegularMessage {
            text,
            id,
            reply_to_message_id,
            forced_to_reply: false,
        }
    }

    pub fn with_forced_reply(mut self) -> RegularMessage {
        self.forced_to_reply = true;
        self
    }
}

impl MoranoMessage for RegularMessage {
    fn to_msg_params(&self) -> SendMessageParams {
        let mut params = SendMessageParams::new(ChatId::Integer(self.id as i64), self.text.clone());
        params.reply_to_message_id = self.reply_to_message_id;

        if self.forced_to_reply {
            params.reply_markup = Some(ReplyMarkup::ForceReply(ForceReply::new(true)));
        }

        params
    }

    fn send_to_chat(&self, api: &Api) -> Result<()> {
        if let Err(e) = api.send_message(&self.to_msg_params()) {
            return Err(anyhow!("{:?}", e));
        }

        Ok(())
    }
}
