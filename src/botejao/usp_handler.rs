use webdriver_client::firefox::GeckoDriver;
use teleborg::{Bot, Command, ParseMode, Updater};
use teleborg::objects::{Update, Message};
use teleborg::error::Error;
use scraper::{Selector};
use encoding::{Encoding};
use std::io::{Write};
use webdriver_client::{Driver, DriverSession};
use webdriver_client::messages::LocationStrategy;
use std;
use time;
pub struct UspHandler {
    geckodriver: GeckoDriver,
    menu_website: String
}

impl Command for UspHandler {
    fn execute(&mut self, bot: &Bot, update: Update, args: Option<Vec<&str>>) {
        self.post_to_usp(bot, update, args);
    }
}

impl UspHandler {

    pub fn new() -> UspHandler {
        UspHandler{
            geckodriver: GeckoDriver::build()
                .firefox_binary("/usr/bin/firefox")
                .spawn().unwrap(),
            menu_website: "https://uspdigital.usp.br/rucard/Jsp/cardapioSAS.jsp?codrtn=6".to_string()
        }
    }
    pub fn post_to_usp(&self, bot: &Bot, update: Update, args: Option<Vec<&str>>) {
        info!("Got request for USPs menu!");
        info!("Opening session!");
        let sess = self.geckodriver.session().unwrap();
        info!("Going to site!");
        let site = sess.go("https://uspdigital.usp.br/rucard/Jsp/cardapioSAS.jsp?codrtn=6")
            .unwrap();
        for i in 0..8 {
            if UspHandler::site_loaded(&sess){
                info!("Parsing site!");
                let menu = UspHandler::get_todays_lunch_menu_from_site(time::now().tm_wday, &sess);
                info!("Replying!");

                let response = UspHandler::reply_to_message_as_markdown(&bot, &update, menu.as_str());
                match response {
                    Ok(response) => info!("Successfully sent \n{:?}.", response),
                    Err(e) => error!("Failed to send \n{}, got \n{:?} as response.", menu, e),
                }
            }else {
                info!("Sleeping for 250 ms for the {} time!", i);
                let quarter_sec = std::time::Duration::from_millis(250);
                std::thread::sleep(quarter_sec);
            }
        }

        error!("Could not load site, timeout");



    }

    fn site_loaded(session: &DriverSession) -> bool{
        return !session.find_element("#almocoSegunda", LocationStrategy::Css)
            .unwrap()
            .text()
            .unwrap().is_empty();
    }
    pub fn get_todays_lunch_menu_from_site(day_since_sunday: i32, session: &DriverSession) -> String{
        match  day_since_sunday {
            1 => format!("*{}*:\n{}" , session.find_element("#diaSegunda", LocationStrategy::Css)
                .unwrap()
                .text()
                .unwrap(), session.find_element("#almocoSegunda", LocationStrategy::Css)
                             .unwrap()
                             .text()
                             .unwrap()),
            2 => format!("*{}* \n {}" , session.find_element("#diaTerca", LocationStrategy::Css)
                .unwrap()
                .text()
                .unwrap(), session.find_element("#almocoTerca", LocationStrategy::Css)
                             .unwrap()
                             .text()
                             .unwrap()),
            3 => format!("*{}* \n {}" , session.find_element("#diaQuarta", LocationStrategy::Css)
                .unwrap()
                .text()
                .unwrap(), session.find_element("#almocoQuarta", LocationStrategy::Css)
                             .unwrap()
                             .text()
                             .unwrap()),
            4 => format!("*{}* \n {}" , session.find_element("#diaQuinta", LocationStrategy::Css)
                .unwrap()
                .text()
                .unwrap(), session.find_element("#almocoQuinta", LocationStrategy::Css)
                             .unwrap()
                             .text()
                             .unwrap()),
            5 => format!("*{}* \n {}" , session.find_element("#diaSexta", LocationStrategy::Css)
                .unwrap()
                .text()
                .unwrap(), session.find_element("#almocoSexta", LocationStrategy::Css)
                             .unwrap()
                             .text()
                             .unwrap()),
            _ => "".to_string()
        }
    }

    fn reply_to_message_as_markdown(
        bot: &Bot,
        update: &Update,
        text: &str,
    ) -> Result<Message, Error> {
        let message = update.clone().message.unwrap();
        let message_id = message.message_id;
        let chat_id = message.chat.id;
        bot.send_message(
            &chat_id,
            text,
            Some(&ParseMode::Markdown),
            None,
            None,
            Some(&message_id),
            None,
        )
    }
}