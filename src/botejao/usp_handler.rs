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
use std::time::Duration;
use std::time::Instant;
use std::thread;
use std::sync::{Arc, RwLock};
pub struct UspHandler {
    geckodriver: GeckoDriver,
    menu_website: String,
    cached_response_struct: Arc<RwLock<CachedResponse>>,
}

struct CachedResponse{
    cached_response: String,
    time_when_cache_was_updated: std::time::Instant,
}
impl UspHandler {
    fn execute(&mut self, bot: &Bot, update: Update, args: Option<Vec<&str>>) {
        self.post_to_usp(bot, update, args);
    }
}

pub struct ArcUspHandler(pub Arc<UspHandler>);

impl Command for ArcUspHandler{
    fn execute(&mut self, bot: &Bot, update: Update, args: Option<Vec<&str>>) {
        self.0.post_to_usp(bot, update, args);
    }
}
impl ArcUspHandler{

    pub fn new()->ArcUspHandler{
        ArcUspHandler(Arc::new(UspHandler::new()))
    }
    pub fn start_updating(&self){
        let a = self.0.clone();
        thread::spawn(move || {
            loop {
                a.update_cached_response();
                thread::sleep(Duration::from_secs(60 * 10));
            }
        });
    }
}

impl UspHandler {

    pub fn new() -> UspHandler {
        UspHandler{
            geckodriver: GeckoDriver::build()
                .firefox_binary("/usr/bin/firefox")
                .spawn().unwrap(),
            menu_website: "https://uspdigital.usp.br/rucard/Jsp/cardapioSAS.jsp?codrtn=6".to_string(),
            cached_response_struct: Arc::new(RwLock::new(CachedResponse{
                cached_response: "".to_string(),
                time_when_cache_was_updated: Instant::now()
            }))

        }
    }


    pub fn post_to_usp(&self, bot: &Bot, update: Update, args: Option<Vec<&str>>) {
        info!("Got request for USPs menu!");
        info!("Sending cached response");
        let cached_response = &self.cached_response_struct.read().unwrap().cached_response;
        let response = UspHandler::reply_to_message_as_markdown(&bot, &update, cached_response.as_str());
        match response {
            Ok(response) => info!("Successfully sent \n{:?}.", response),
            Err(e) => error!("Failed to send \n{}, got \n{:?} as response.", cached_response, e),
        }
    }




    pub fn update_cached_response(&self){
        let mut menu = String::new();
        info!("Opening session!");
        let sess = self.geckodriver.session().unwrap();
        info!("Going to site!");
        sess.go("https://uspdigital.usp.br/rucard/Jsp/cardapioSAS.jsp?codrtn=6").unwrap();
        info!("Got site response, checking if he is already loaded!");
        for i in 0..20 {
            if UspHandler::site_loaded(&sess) {
                info!("Parsing site!");
                menu = UspHandler::get_todays_menu_formated(time::now().tm_wday, &sess);
                if !menu.is_empty() {
                    info!("Was loaded!");
                    break;
                }
            } else {
                info!("Was NOT loaded.");
                info!("Sleeping for 500 ms for the {} time!", i);
                let quarter_sec = Duration::from_millis(500);
                std::thread::sleep(quarter_sec);
            }
        }
        if menu.is_empty(){
            error!("Could not load site, timeout");
            return;
        }

        info!("Updating response cache");
        {
            let mut cached_struct = self.cached_response_struct.write().unwrap();
            cached_struct.cached_response = menu;
            cached_struct.time_when_cache_was_updated = Instant::now();
        }
    }

    fn site_loaded(session: &DriverSession) -> bool{
        return !session.find_element("#almocoSegunda", LocationStrategy::Css)
            .unwrap()
            .text()
            .unwrap().is_empty();
    }
    pub fn get_todays_menu_formated(day_since_sunday: i32, session: &DriverSession) -> String{
        match  day_since_sunday {
            1 => format!("*{}*:\n\n*Almoço:*\n{}\n\n*Jantar:*\n{}" ,
                         session.find_element("#diaSegunda", LocationStrategy::Css).unwrap().text().unwrap(),
                         session.find_element("#almocoSegunda", LocationStrategy::Css).unwrap().text().unwrap(),
                         session.find_element("#jantarSegunda", LocationStrategy::Css).unwrap().text().unwrap()),
            2 => format!("*{}*:\n\n*Almoço:*\n{}\n\n*Jantar:*\n{}" ,
                         session.find_element("#diaTerca", LocationStrategy::Css).unwrap().text().unwrap(),
                         session.find_element("#almocoTerca", LocationStrategy::Css).unwrap().text().unwrap(),
                         session.find_element("#jantarTerca", LocationStrategy::Css).unwrap().text().unwrap()),
            3 => format!("*{}*:\n\n*Almoço:*\n{}\n\n*Jantar:*\n{}" ,
                         session.find_element("#diaQuarta", LocationStrategy::Css).unwrap().text().unwrap(),
                         session.find_element("#almocoQuarta", LocationStrategy::Css).unwrap().text().unwrap(),
                         session.find_element("#jantarQuarta", LocationStrategy::Css).unwrap().text().unwrap()),
            4 => format!("*{}*:\n\n*Almoço:*\n{}\n\n*Jantar:*\n{}" ,
                         session.find_element("#diaQuinta", LocationStrategy::Css).unwrap().text().unwrap(),
                         session.find_element("#almocoQuinta", LocationStrategy::Css).unwrap().text().unwrap(),
                         session.find_element("#jantarQuinta", LocationStrategy::Css).unwrap().text().unwrap()),
            5 => format!("*{}*:\n\n*Almoço:*\n{}\n\n*Jantar:*\n{}" ,
                         session.find_element("#diaSexta", LocationStrategy::Css).unwrap().text().unwrap(),
                         session.find_element("#almocoSexta", LocationStrategy::Css).unwrap().text().unwrap(),
                         session.find_element("#jantarSexta", LocationStrategy::Css).unwrap().text().unwrap()),
            _ => format!("*{}*:\n\n*Almoço:*\n{}\n\n*Jantar:*\n{}" ,
                         session.find_element("#diaSegunda", LocationStrategy::Css).unwrap().text().unwrap(),
                         session.find_element("#almocoSegunda", LocationStrategy::Css).unwrap().text().unwrap(),
                         session.find_element("#jantarSegunda", LocationStrategy::Css).unwrap().text().unwrap())
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