//extern crate futures;
extern crate encoding;
extern crate flexi_logger;
#[macro_use]
extern crate log;
extern crate reqwest;
extern crate scraper;
extern crate teleborg;
extern crate time;
extern crate webdriver_client;
use teleborg::*;
mod unicamp_handler;
//mod usp_handler;
use unicamp_handler::UnicampHandler;
use teleborg::{Bot};
use teleborg::objects::Update;
//use usp_handler::ArcUspHandler;
pub struct Botejao {
    unicamp_handler: UnicampHandler,
//    usp_handler: ArcUspHandler,
    telegram_api: Bot,
}




impl Botejao {
    pub fn new(bot_token: String) -> Botejao {
        return Botejao {
            unicamp_handler: UnicampHandler::new(),
//            usp_handler: ArcUspHandler::new(),
            telegram_api: Bot::new(bot_token).unwrap(),
        };
    }

    fn get_updates_list(&self) -> Vec<Update>{
        let updates_channel = self.telegram_api.get_updates_channel();
        loop {
            let possible_updates_list = updates_channel.recv();
            match possible_updates_list {
                Ok(update_list) => return update_list,
                Err(e) => {
                    error!("Error while getting updates list from Teleborg: {}", e);
                }
            };
        }
    }

    fn get_response_for_update(&self, update: &Update) -> Option<String> {
        let update_msg_text = match update.message.as_ref().and_then(|msg| msg.text.as_ref()) {
            Some(text) => text,
            None => {
                error!("Update with no message text!");
                return None;
            }
        };
        match update_msg_text.as_str() {
            "/unicamp" | "/unicamp@BotejaoBot" => {
//                return Some(self.unicamp_handler.get_fresh_unicamp_menu().unwrap())
                return Some(self.unicamp_handler.get_unicamp_menu());
            },
            _ => {
                return Some("O Bot está sob reformas, somente /unicamp@BotejaoBot disponível no momento.".to_string());
            }
        }
    }

    fn handle_update(&self, update: &Update){
        let message = match update.message.as_ref() {
            Some(msg) => msg,
            None => {
                error!("Update with no message : {:?}", update);
                return;
            }
        };

        if let Some(response) = self.get_response_for_update(update){
            let outgoing_message = OutgoingMessage::new(message.chat.id,&response);
            self.telegram_api.send_msg(outgoing_message);
        }else{
            info!("Had no response to send!");
        }
    }

    pub fn start(mut self) {
//        self.usp_handler.start_updating();

        self.telegram_api.start_getting_updates();

        loop {
            let updates = self.get_updates_list();
            for update in updates {
                self.handle_update(&update);
            }
        }
    }



}
