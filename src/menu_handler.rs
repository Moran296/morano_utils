use frankenstein::{Api, BotCommand, DeleteMyCommandsParams, SetMyCommandsParams, TelegramApi};

type CommandName = &'static str;
type CommandDescription = &'static str;

#[derive(Debug)]
pub struct MoranoMenu {
    menu: Vec<(CommandName, CommandDescription)>,
}

impl MoranoMenu {
    pub fn new() -> Self {
        MoranoMenu { menu: vec![] }
    }

    pub fn from(menu: Vec<(&'static str, &'static str)>) -> Self {
        MoranoMenu { menu }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, (&str, &str)> {
        self.menu.iter()
    }

    pub fn remove(&mut self, command: &'static str) {
        for (i, c) in self.menu.iter_mut().enumerate() {
            if c.0 == command {
                self.menu.remove(i);
                break;
            }
        }
    }

    pub fn commit(&self, api: &Api) {
        if let Err(e) = api.delete_my_commands(&DeleteMyCommandsParams::new()) {
            println!("error deleting menu {:?}", e);
        }

        let v: Vec<BotCommand> = self
            .menu
            .iter()
            .map(|x| BotCommand::new(x.0.to_string(), x.1.to_string()))
            .collect();

        let params = SetMyCommandsParams::new(v);
        if let Err(e) = api.set_my_commands(&params) {
            println!("error setting menu {:?}", e);
        }
    }
}
