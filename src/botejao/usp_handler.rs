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
use std::fmt;
use std::process;

pub struct UspHandler {
    geckodriver: GeckoDriver,
    pub arc_usp_central_replier: ArcUspReplier,
    pub arc_usp_prefeitura_replier: ArcUspReplier,
    pub arc_usp_fisica_replier: ArcUspReplier,
    pub arc_usp_quimica_replier: ArcUspReplier,
}

pub struct UspReplier {
    cached_response_struct: RwLock<CachedResponse>,
    menu_website: String,
}

pub struct ArcUspReplier {
    pub arc_usp_replier: Arc<UspReplier>,
}

impl Clone for ArcUspReplier {
    fn clone(&self) -> Self {
        ArcUspReplier {
            arc_usp_replier: self.arc_usp_replier.clone(),
        }
    }
}

struct CachedResponse{
    cached_response: String,
    time_when_cache_was_updated: std::time::Instant,
}

pub struct ArcUspHandler {
    pub actual_arc_usp: Arc<UspHandler>,
}

impl Command for ArcUspReplier {
    fn execute(&mut self, bot: &Bot, update: Update, args: Option<Vec<&str>>) {
        self.arc_usp_replier.post_to_usp(bot, update, args);
    }
}

impl UspReplier {
    pub fn post_to_usp(&self, bot: &Bot, update: Update, args: Option<Vec<&str>>) {
        info!("Got request for USPs menu");
        let username = update.message.as_ref()
            .and_then(|msg| msg.from.as_ref())
            .and_then(|from| from.username.as_ref());

        match username {
            Some(username) => info!("Got message from user: {}", username),
            None => error!("The following update did not contain an username: {:?}", update)
        }
        info!("Sending cached response");
        let cached_response = &self.cached_response_struct.read().unwrap().cached_response;
        let response = UspHandler::reply_to_message_as_markdown(&bot, &update, cached_response.as_str());
        match response {
            Ok(_) => info!("Successfully sent USPs menu."),
            Err(e) => error!("Failed to send \n{}, got \n{:?} as response.", cached_response, e),
        }
    }
}

impl ArcUspHandler{

    pub fn new()->ArcUspHandler {
        ArcUspHandler {
            actual_arc_usp: Arc::new(UspHandler::new())
        }
    }
    pub fn start_updating(&self){
        let usp_handler_ref_copy = self.actual_arc_usp.clone();
        thread::spawn(move || {
            loop {
                usp_handler_ref_copy.update_cached_response();
                thread::sleep(Duration::from_secs(60 * 60));
            }
        });
    }
}

impl UspHandler {

    pub fn new() -> UspHandler {
        UspHandler {
            geckodriver: GeckoDriver::build()
                .firefox_binary("/usr/bin/firefox")
//                .firefox_binary("/Applications/FirefoxDeveloperEdition.app/Contents/MacOS/firefox-bin")
                .spawn().unwrap(),
            arc_usp_central_replier: ArcUspReplier {
                arc_usp_replier: Arc::new(UspReplier {
                cached_response_struct: RwLock::new(CachedResponse{
                    cached_response: "".to_string(),
                    time_when_cache_was_updated: Instant::now(),
                }),
                menu_website: "https://uspdigital.usp.br/rucard/Jsp/cardapioSAS.jsp?codrtn=6".to_string(),
            })},
            arc_usp_prefeitura_replier: ArcUspReplier {
                arc_usp_replier: Arc::new(UspReplier {
                    cached_response_struct: RwLock::new(CachedResponse{
                        cached_response: "".to_string(),
                        time_when_cache_was_updated: Instant::now(),
                    }),
                    menu_website: "https://uspdigital.usp.br/rucard/Jsp/cardapioSAS.jsp?codrtn=7".to_string(),
                })}
            ,arc_usp_fisica_replier: ArcUspReplier {
            arc_usp_replier: Arc::new(UspReplier {
                cached_response_struct: RwLock::new(CachedResponse{
                    cached_response: "".to_string(),
                    time_when_cache_was_updated: Instant::now(),
                }),
                menu_website: "https://uspdigital.usp.br/rucard/Jsp/cardapioSAS.jsp?codrtn=8".to_string(),
            })},
            arc_usp_quimica_replier: ArcUspReplier {
                arc_usp_replier: Arc::new(UspReplier {
                    cached_response_struct: RwLock::new(CachedResponse{
                        cached_response: "".to_string(),
                        time_when_cache_was_updated: Instant::now(),
                    }),
                    menu_website: "https://uspdigital.usp.br/rucard/Jsp/cardapioSAS.jsp?codrtn=9".to_string(),
                })}
        }
    }

    pub fn update_cached_response(&self){
        info!("Checking if Firefox is UP");
        loop {
            let output = String::from_utf8
                (std::process::Command::new("pidof")
                .arg("firefox")
                .output()
                .unwrap()
                .stdout)
                .unwrap();
            if !output.is_empty() {
                std::process::Command::new("kill")
                    .arg(&output.trim())
                    .spawn().expect("Could not kill");
                error!("Killed firefox with PID {} !", output);
            } else {
                info!("Did not need to kill firefox!");
                break;
            }
        }

        info!("Opening session!");
        let sess = match self.geckodriver.session() {
            Ok(sess) => sess,
            Err(err) => {
                error!("Could not open DriverSession, got error {}", err);
                return;
            },
        };
        info!("Going to sites!");
        let websites = vec![
            self.arc_usp_central_replier.arc_usp_replier.as_ref(),
            self.arc_usp_prefeitura_replier.arc_usp_replier.as_ref(),
            self.arc_usp_fisica_replier.arc_usp_replier.as_ref(),
            self.arc_usp_quimica_replier.arc_usp_replier.as_ref(),
        ];
        for arc_usp_replier in websites.iter() {
            let mut menu = String::new();
            info!("Going to site {}!", arc_usp_replier.menu_website);
            match sess.go(arc_usp_replier.menu_website.as_str()){
                Ok(_) => (),
                Err(err) => {
                    error!("Could not go to site {}, got error {}",arc_usp_replier.menu_website, err);
                    return;
                }

            }
            info!("Got site response, checking if he is already loaded!");
            for i in 0..30 {
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
                    let quarter_sec = Duration::from_secs(1);
                    std::thread::sleep(quarter_sec);
                }
            }
            if menu.is_empty() {
                error!("Could not load site, timeout");
                return;
            }

            info!("Updating response cache");
            {
                let mut cached_struct = arc_usp_replier.cached_response_struct.write().unwrap();
                cached_struct.cached_response = menu;
                cached_struct.time_when_cache_was_updated = Instant::now();
            }
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