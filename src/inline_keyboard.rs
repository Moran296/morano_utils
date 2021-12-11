use crate::traits::MoranoMessage;
use anyhow::{anyhow, Result};
use frankenstein::{
    Api, CallbackQuery, ChatId, InlineKeyboardButton, ReplyMarkup, SendMessageParams, TelegramApi,
};

#[allow(dead_code)]
#[allow(unused_imports)]
pub enum ButtonAction<'a> {
    Callback(String, &'a dyn Fn(&'a CallbackQuery)),
    Query(String),
    Link(String),
}

pub type Button<'a> = (String, ButtonAction<'a>);
type ButtonRow<'a> = Vec<Button<'a>>;

pub struct InlineKeyboardMessage<'a> {
    text: String,
    id: u64,
    buttons: Vec<ButtonRow<'a>>,
    button_row: usize,
}

impl<'a> InlineKeyboardMessage<'a> {
    ///  new message with reply keyboard
    /// # Arguments
    /// * `text` - text of the message
    pub fn new(id: u64, text: &str) -> Self {
        InlineKeyboardMessage {
            text: text.to_string(),
            id,
            buttons: vec![ButtonRow::new()],
            button_row: 0,
        }
    }

    /// Add a button with a callback to the current row
    ///  # Arguments
    /// * `text` - text of the button
    /// * `callback_name` - callback query value to run callback on
    /// * `callback` - callback function
    pub fn with_callback_button(
        mut self,
        text: &str,
        callback_name: &str,
        callback: &'a dyn Fn(&'a CallbackQuery),
    ) -> Self {
        self.buttons[self.button_row].push((
            text.to_string(),
            ButtonAction::Callback(callback_name.to_string(), callback),
        ));
        self
    }

    /// Add a button with a link to the current row
    /// * `text` - text of the button
    /// * `link` - url to redirect to
    pub fn with_link_button(mut self, text: &str, link: &str) -> Self {
        self.buttons[self.button_row]
            .push((text.to_string(), ButtonAction::Link(link.to_string())));
        self
    }

    /// Add a button with a query to the current row
    /// * `text` - text of the button
    /// * `callback` - callback query value
    pub fn with_query_button(mut self, text: &str, callback: &str) -> Self {
        self.buttons[self.button_row]
            .push((text.to_string(), ButtonAction::Query(callback.to_string())));
        self
    }

    /// Moves to next row of keyboard
    pub fn next_row(mut self) -> Self {
        self.button_row += 1;
        self.buttons.push(ButtonRow::new());
        self
    }

    ///Iterate all buttons in the keyboard
    pub fn iter(&'a self) -> ButtonIterator {
        ButtonIterator {
            buttons: &self.buttons,
            row: 0,
            column: 0,
        }
    }

    //TODO iter_mut

    ///Runs a callback for a button in the query if exists and returns wheather the button was found and run
    pub fn run_callback(&'a self, query: &'a CallbackQuery) -> bool {
        if query.data.is_none() {
            return false;
        }

        let text = query.data.as_ref().unwrap();
        for g in self.iter() {
            if let ButtonAction::Callback(ref name, cb) = g.1 {
                if name == text {
                    cb(query);
                    return true;
                }
            }
        }
        false
    }
}

/// Implemantion of MoranoMessage on InlineKeyboardMessage
impl MoranoMessage for InlineKeyboardMessage<'_> {
    fn to_msg_params(&self) -> SendMessageParams {
        let mut params = SendMessageParams::new(ChatId::Integer(self.id as i64), self.text.clone());

        let mut markup: frankenstein::InlineKeyboardMarkup =
            frankenstein::InlineKeyboardMarkup::new(vec![vec![]]);

        let mut row = 0;
        for button_row in self.buttons.iter() {
            for my_button in button_row {
                let mut api_button = InlineKeyboardButton::new(my_button.0.clone());
                match &my_button.1 {
                    ButtonAction::Query(ref callback) => {
                        api_button.set_callback_data(Some(callback.clone()));
                    }
                    ButtonAction::Callback(ref callback, _) => {
                        api_button.set_callback_data(Some(callback.clone()));
                    }
                    ButtonAction::Link(ref link) => {
                        api_button.set_url(Some(link.clone()));
                    }
                }

                markup.inline_keyboard[row].push(api_button);
            }

            markup.inline_keyboard.push(vec![]);
            row += 1;
        }

        params.set_reply_markup(Some(ReplyMarkup::InlineKeyboardMarkup(markup)));
        params
    }

    fn send_to_chat(&self, api: &Api) -> Result<()> {
        if let Err(e) = api.send_message(&self.to_msg_params()) {
            return Err(anyhow!("{:?}", e));
        }

        Ok(())
    }
}

pub struct ButtonIterator<'a> {
    buttons: &'a Vec<ButtonRow<'a>>,
    row: usize,
    column: usize,
}

impl<'a> Iterator for ButtonIterator<'a> {
    type Item = &'a Button<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.row >= self.buttons.len() {
            return None;
        }

        let row = self.buttons.get(self.row)?;
        let res = row.get(self.column)?;

        if self.column + 1 < row.len() {
            self.column += 1;
        } else {
            self.row += 1;
            self.column = 0;
        }

        Some(res)
    }
}
