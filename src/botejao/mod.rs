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
use teleborg::{Bot, Dispatcher, Updater};
use teleborg::objects::Update;
use usp_handler::ArcUspHandler;
pub struct Botejao {
    unicamp_handler: UnicampHandler,
    usp_handler: ArcUspHandler,
    bot_dispatcher: Dispatcher,
    bot_token: String,
}




impl Botejao {
    pub fn new(
        bot_token: String,
    ) -> Botejao {
        return Botejao {
            unicamp_handler: UnicampHandler::new(),
            usp_handler: ArcUspHandler::new(),
            bot_dispatcher: Dispatcher::new(),
            bot_token,
        };
    }

    pub fn start(mut self) {

        self.usp_handler.start_updating();


        let arc_usp_handler = self.usp_handler.actual_arc_usp.clone();


        self.bot_dispatcher
            .add_command_handler("unicamp", self.unicamp_handler, false);
        self.bot_dispatcher
            .add_command_handler("usp_central", arc_usp_handler.arc_usp_central_replier.clone(), false);
        self.bot_dispatcher
            .add_command_handler("usp_fisica", arc_usp_handler.arc_usp_fisica_replier.clone(), false);
        self.bot_dispatcher
            .add_command_handler("usp_prefeitura", arc_usp_handler.arc_usp_prefeitura_replier.clone(), false);
        self.bot_dispatcher
            .add_command_handler("usp_quimica", arc_usp_handler.arc_usp_quimica_replier.clone(), false);
        self.bot_dispatcher
            .add_command_handler("help", Botejao::send_help, false);
        self.bot_dispatcher
            .add_command_handler("ajuda", Botejao::send_help, false);
        self.bot_dispatcher
            .add_command_handler("start", Botejao::send_help, false);
        Updater::start(
            Some(self.bot_token.clone()),
            Option::Some(2),
            Option::Some(30),
            None,
            self.bot_dispatcher,
        );
    }

    fn send_help(bot: &Bot, update: Update, _: Option<Vec<&str>>) {
        info!("Got request for help!");
        let username = update.message.as_ref()
            .and_then(|msg| msg.from.as_ref())
            .and_then(|from| from.username.as_ref());

        match username {
            Some(username) => info!("Got message from user: {}", username),
            None => error!("The following update did not contain an username: {:?}", update)
        }
        info!("Replying to it");
        let help_msg =
            "Esse bot retorna o menu de restaurantes universitarios. Use o comando /unicamp@BotejaoBot ou /usp_central@BotejaoBot para testa-lo.";
        let response = bot.reply_to_message(&update, help_msg);
        match response {
            Ok(_) => info!("Successfully sent the help message."),
            Err(e) => error!("Failed to send \n{}, got \n{:?} as response.", help_msg, e),
        }
    }

}
