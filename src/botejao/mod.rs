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

mod unicamp_handler;
mod usp_handler;
use unicamp_handler::UnicampHandler;
use encoding::{Encoding};
use std::io::{Write};
use std::path::{PathBuf};
use teleborg::{Bot, Dispatcher, ParseMode, Updater};
use teleborg::objects::Update;
use scraper::{Selector};
use usp_handler::UspHandler;
pub struct Botejao {
    unicamp_handler: UnicampHandler,
    usp_handler: UspHandler,
    bot_dispatcher: Dispatcher,
    bot_token: String,
    base_bot_url: String,
}




impl Botejao {
    pub fn new(
        bot_token: String,
    ) -> Botejao {
        return Botejao {
            unicamp_handler: UnicampHandler::new(),
            usp_handler: UspHandler::new(),
            bot_dispatcher: Dispatcher::new(),
            bot_token,
            base_bot_url: "https://api.telegram.org/bot".to_string(),
        };
    }

    pub fn start(mut self) {
        let bot_url = [self.base_bot_url.as_str(), &self.bot_token.clone()].concat();
        let thread_bot = Bot::new(bot_url).expect("The bot token appears to be invalid.");

        self.bot_dispatcher
            .add_command_handler("unicamp", self.unicamp_handler, false);
        self.bot_dispatcher
            .add_command_handler("usp_central", self.usp_handler, false);
        self.bot_dispatcher
            .add_command_handler("help", Botejao::send_help, false);
        self.bot_dispatcher
            .add_command_handler("ajuda", Botejao::send_help, false);
        self.bot_dispatcher
            .add_command_handler("start", Botejao::send_help, false);
        Updater::start(
            Some(self.bot_token.clone()),
            None,
            None,
            None,
            self.bot_dispatcher,
        );
    }

    fn send_help(bot: &Bot, update: Update, _: Option<Vec<&str>>) {
        info!("Got request for help! {:?}", update);
        info!("Replying to it");
        let help_msg =
            "Esse bot retorna o menu de restaurantes universitarios. Use o comando /unicamp@BotejaoBot ou /usp_central@BotejaoBot para testa-lo.";
        let response = bot.reply_to_message(&update, help_msg);
        match response {
            Ok(response) => info!("Successfully sent \n{:?}.", response),
            Err(e) => error!("Failed to send \n{}, got \n{:?} as response.", help_msg, e),
        }
    }

}
