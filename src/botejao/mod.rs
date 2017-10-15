//extern crate futures;
extern crate teleborg;
extern crate scraper;
extern crate encoding;
extern crate reqwest;
#[macro_use]
extern crate log;
extern crate flexi_logger;
extern crate time;

use encoding::{Encoding, DecoderTrap};
use encoding::all::ISO_8859_1;
use std::io::{Read, Write};
use std::fs::File;
use std::path::{Path, PathBuf};
use std::thread;
use teleborg::{Dispatcher, Bot, Updater, ParseMode};
use teleborg::objects::{Update};
use scraper::{Selector, Html};
use std::fs::OpenOptions;

pub struct Botejao {
    broadcast_groups: Vec<i64>,
    path_to_last_menu_file: PathBuf,
    bot_dispatcher: Dispatcher,
    bot_token: String,
    base_bot_url: String
}

impl Botejao{

    pub fn new(bot_token: String, broadcast_groups: Vec<i64>, path_to_last_menu_file: PathBuf) -> Botejao {
        return Botejao{
            broadcast_groups,
            path_to_last_menu_file,
            bot_dispatcher: Dispatcher::new(),
            bot_token,
            base_bot_url: "https://api.telegram.org/bot".to_string()
        }
    }

    pub fn start(mut self){
        let bot_url = [self.base_bot_url.as_str(), &self.bot_token.clone()].concat();
        let thread_bot = Bot::new(bot_url).expect("The bot token appears to be invalid.");
        let path_to_last_menu_file = self.path_to_last_menu_file.clone();
        let broadcast_groups_thread = self.broadcast_groups.clone();
        thread::spawn(move || {
            let one_hour = std::time::Duration::from_secs(60*60);
            loop {
                info!("Begin of Broadcast loop, checking for site update.");
                Botejao::broadcast_if_needed(&thread_bot, &path_to_last_menu_file, broadcast_groups_thread.get(0).unwrap());
                std::thread::sleep(one_hour);
            }
        });

        self.bot_dispatcher.add_command_handler("bandejao", Botejao::send_menu, false);
        self.bot_dispatcher.add_command_handler("bandeco", Botejao::send_menu, false);
        self.bot_dispatcher.add_command_handler("ru", Botejao::send_menu, false);
        Updater::start(Some(self.bot_token.clone()), None, None, None, self.bot_dispatcher);
    }

    fn send_menu(bot: &Bot, update: Update, _: Option<Vec<&str>>) {
        info!("Got request for menu! {:?}", update);
        info!("Replying to it");
        let menu = Botejao::get_menu("https://www.prefeitura.unicamp.br/apps/site/cardapio.php".to_string()).unwrap();
        let response = Botejao::reply_to_message_as_markdown(&bot, &update, menu.as_str());
        match response {
            Ok(response) => info!("Successfully sent \n{:?}.", response),
            Err(e) => error!("Failed to send \n{}, got \n{:?} as response.", menu, e)
        }
    }

    pub fn reply_to_message_as_markdown(bot: &Bot, update: &Update, text: &str) -> Result<teleborg::objects::Message, teleborg::error::Error> {
        let message = update.clone().message.unwrap();
        let message_id = message.message_id;
        let chat_id = message.chat.id;
        bot.send_message(&chat_id, text, Some(&ParseMode::Markdown), None, None, Some(&message_id), None)
    }

    fn remove_spaces_and_tabs(input: String) -> String {
        let mut input = input.replace("\n","").replace("\t","");
        input.push('\n');
//        info!("Outputing: {:?}", input);
        return input;
    }

    fn apply_selectors(selector: &Selector, fragment: &Html) -> String{
        fragment.select(&selector).next().unwrap().text().collect::<String>()
    }


    fn get_site_response(site :String) -> reqwest::Result<reqwest::Response>{
        reqwest::get(site.as_str())
    }

    fn filter_response(resp: &mut reqwest::Response) -> std::result::Result<String, String> {
        let mut body = Vec::new();
        let body_str;



        match resp.read_to_end(&mut body) {
            Ok(_) => {
                body_str = ISO_8859_1.decode(&*body, DecoderTrap::Strict).unwrap();
            },
            Err(e) => {
                return Err(format!("Error reading site response {}", e));
            }
        }
        let fragment = Html::parse_fragment(&body_str);

        let day_selector = Selector::parse("#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > p").unwrap();

        let selector_breakfast = Selector::parse("#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(2)").unwrap();


        let mut lunch_selectors = Vec::new();
        lunch_selectors.push(Selector::parse("#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td:nth-child(1) > table > tbody > tr:nth-child(1) > td").unwrap());
        lunch_selectors.push(Selector::parse("#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td:nth-child(1) > table > tbody > tr:nth-child(2) > td").unwrap());
        lunch_selectors.push(Selector::parse("#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td:nth-child(1) > table > tbody > tr:nth-child(3) > td").unwrap());
        lunch_selectors.push(Selector::parse("#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td").unwrap());
        lunch_selectors.push(Selector::parse("#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td:nth-child(1) > table > tbody > tr:nth-child(5) > td").unwrap());
        lunch_selectors.push(Selector::parse("#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td:nth-child(1) > table > tbody > tr:nth-child(6) > td").unwrap());

        let mut veg_lunch_selectors = Vec::new();

        veg_lunch_selectors.push(Selector::parse("#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td:nth-child(2) > table > tbody > tr:nth-child(1) > td").unwrap());
        veg_lunch_selectors.push(Selector::parse("#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td:nth-child(2) > table > tbody > tr:nth-child(2) > td").unwrap());
        veg_lunch_selectors.push(Selector::parse("#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td:nth-child(2) > table > tbody > tr:nth-child(4) > td").unwrap());
        veg_lunch_selectors.push(Selector::parse("#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td:nth-child(2) > table > tbody > tr:nth-child(5) > td").unwrap());
        veg_lunch_selectors.push(Selector::parse("#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td:nth-child(2) > table > tbody > tr:nth-child(6) > td").unwrap());

        let mut dinner_selectors = Vec::new();

        dinner_selectors.push(Selector::parse("#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td:nth-child(3) > table > tbody > tr:nth-child(1) > td").unwrap());
        dinner_selectors.push(Selector::parse("#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td:nth-child(3) > table > tbody > tr:nth-child(2) > td").unwrap());
        dinner_selectors.push(Selector::parse("#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td:nth-child(3) > table > tbody > tr:nth-child(3) > td").unwrap());
        dinner_selectors.push(Selector::parse("#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td:nth-child(3) > table > tbody > tr:nth-child(4) > td").unwrap());
        dinner_selectors.push(Selector::parse("#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td:nth-child(3) > table > tbody > tr:nth-child(5) > td").unwrap());
        dinner_selectors.push(Selector::parse("#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td:nth-child(3) > table > tbody > tr:nth-child(6) > td").unwrap());

        let mut veg_dinner_selectors = Vec::new();
        veg_dinner_selectors.push(Selector::parse("#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td:nth-child(4) > table > tbody > tr:nth-child(1) > td").unwrap());
        veg_dinner_selectors.push(Selector::parse("#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td:nth-child(4) > table > tbody > tr:nth-child(2) > td").unwrap());
        veg_dinner_selectors.push(Selector::parse("#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td:nth-child(4) > table > tbody > tr:nth-child(4) > td").unwrap());
        veg_dinner_selectors.push(Selector::parse("#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td:nth-child(4) > table > tbody > tr:nth-child(5) > td").unwrap());
        veg_dinner_selectors.push(Selector::parse("#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td:nth-child(4) > table > tbody > tr:nth-child(6) > td").unwrap());

        let mut cafe_da_manha = String::from("*Café da Manha:* \n");

        let extracted_break_fast = Botejao::apply_selectors(&selector_breakfast, &fragment);

        cafe_da_manha.push_str(Botejao::remove_spaces_and_tabs(extracted_break_fast).as_str());

        let extracted_day = Botejao::apply_selectors(&day_selector, &fragment);

        let day = format!("*{}*", Botejao::remove_spaces_and_tabs(extracted_day));


        let mut lunch = String::from("*Almoço:* \n");

        for selector in &lunch_selectors{
            lunch.push_str(Botejao::remove_spaces_and_tabs(Botejao::apply_selectors(selector, &fragment)).as_str());
        }

        let mut lunch_veg = String::from("*Almoço Vegetariano:* \n");

        for selector in &veg_lunch_selectors{
            lunch_veg.push_str(Botejao::remove_spaces_and_tabs(Botejao::apply_selectors(selector, &fragment)).as_str());
        }
        let mut dinner = String::from("*Jantar:* \n");

        for selector in &dinner_selectors{
            dinner.push_str(Botejao::remove_spaces_and_tabs(Botejao::apply_selectors(selector, &fragment)).as_str());
        }

        let mut dinner_veg = String::from("*Jantar Vegetariano: *\n");

        for selector in &veg_dinner_selectors{
            dinner_veg.push_str(Botejao::remove_spaces_and_tabs(Botejao::apply_selectors(selector, &fragment)).as_str());
        }
        return Ok(format!("{}\n {}\n{}\n{}\n{}", day, lunch, dinner, lunch_veg, dinner_veg));
    }
    fn get_menu(site: String) -> std::result::Result<String, ()>{
        let mut resp =
            match Botejao::get_site_response(site){
                Ok(response) => response,
                Err(e) => {
                    error!("Could not get site error: {}", e);
                    return Err(());
                }
            };
        return match Botejao::filter_response(&mut resp){
            Ok(menu) => Ok(menu),
            Err(_) => Err(())
        }

    }
    fn needs_to_publish(menu: &str, path_to_last_menu_file: &Path) -> bool {
        return  menu != Botejao::read_menu_from_file(path_to_last_menu_file);
    }
    fn broadcast_if_needed(bot: &Bot, path_to_last_menu_file: &Path, rep_chat_id: &i64){

        let menu =
            match Botejao::get_menu("https://www.prefeitura.unicamp.br/apps/site/cardapio.php".to_string()){
                Ok(menu) => menu,
                Err(_) => {
                    error!("Could not load menu from site");
                    return;
                }
            };
        if Botejao::needs_to_publish(menu.as_str(), &path_to_last_menu_file){
            match Botejao::broadcast(&bot, &menu.as_str(), &rep_chat_id) {
                Ok(_) => {
                    Botejao::write_menu_to_file(&path_to_last_menu_file, &menu);
                    info!("Broadcast successful");
                },
                Err(_) => {
                    error!("Failed to broadcast, not going to update menu file");
                }
            }

        }
    }

    fn write_menu_to_file(path_to_last_menu_file: &Path, menu: &str){
        let mut file = File::create(path_to_last_menu_file).unwrap();
        match file.write(menu.as_bytes()){
            Ok(_) => (),
            Err(e) => error!("Could not write to file {:?}\n Error: {}", path_to_last_menu_file, e)
        }
    }

    fn read_menu_from_file(path_to_last_menu_file: &Path) -> String{
        let mut file =  OpenOptions::new()
            .read(true)
            .open(&path_to_last_menu_file)
            .unwrap();
        let mut menu_from_file = String::new();
        file.read_to_string(&mut menu_from_file).unwrap();
        return menu_from_file;
    }

    fn broadcast(bot: &Bot, message: &str, rep_chat_id: &i64) -> Result<(), ()>{
        let result = bot.send_message(
            rep_chat_id,
            &message,
            Some(&ParseMode::Markdown),
            None,
            None,
            None,
            None,
        );
        match result {
            Ok(response) => {
                info!("Successfully broadcast \n{:?}.", response);
                return Ok(());
            },
            Err(e) => {
                error!("Failed to broadcast, got \n{:?} as response.", e);
                return Err(());
            }
        }
    }

}

