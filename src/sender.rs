use frankenstein::{Api, ChatId, SendMessageParams, TelegramApi};

pub struct MoranoSender {
    api: Api,
    pub chat_id: i64,
}

impl MoranoSender {
    pub fn new(api: Api, chat_id: i64) -> Self {
        MoranoSender { api, chat_id }
    }

    pub fn send(&self, text: String) {
        let params = SendMessageParams::new(ChatId::Integer(self.chat_id), text);
        let res = self.api.send_message(&params);
        if res.is_err() {
            println!("Error sending message: {:?}", res);
        }
    }
}
