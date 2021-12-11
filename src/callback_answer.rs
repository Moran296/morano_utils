use crate::traits::*;
use anyhow::{anyhow, Result};
use frankenstein::{
    AnswerCallbackQueryParams, Api, CallbackQuery, ChatId, SendMessageParams, TelegramApi,
};

pub struct MoranoCBAnswer {
    pub id: String,
    upper_text: Option<String>,
    msg_text: Option<String>,
}

impl MoranoCBAnswer {
    pub fn new(query: &CallbackQuery) -> Self {
        Self {
            id: query.id.clone(),
            upper_text: None,
            msg_text: None,
        }
    }

    pub fn with_upper_text(mut self, text: &str) -> Self {
        self.upper_text = Some(text.to_string());
        self
    }

    pub fn with_message(mut self, text: &str) -> Self {
        self.msg_text = Some(text.to_string());
        self
    }
}

impl MoranoMessage for MoranoCBAnswer {
    fn to_msg_params(&self) -> SendMessageParams {
        if let Some(ref text) = self.msg_text {
            SendMessageParams::new(ChatId::String(self.id.clone()), text.clone())
        } else {
            panic!("No message text provided")
        }
    }

    fn send_to_chat(&self, api: &Api) -> Result<()> {
        let answer = AnswerCallbackQueryParams {
            callback_query_id: self.id.clone(),
            text: self.upper_text.clone(),
            show_alert: None,
            url: None,
            cache_time: None,
        };

        if self.msg_text.is_some() {
            let params = self.to_msg_params();
            if let Err(e) = api.send_message(&params) {
                return Err(anyhow!("{:?}", e));
            }
        }

        if let Err(e) = api.answer_callback_query(&answer) {
            return Err(anyhow!("{:?}", e));
        }

        Ok(())
    }
}
