#![allow(warnings)] // remove when error_chain is fixed
extern crate futures;
extern crate teleborg;

extern crate encoding;
extern crate reqwest;
#[macro_use]
extern crate error_chain;
extern crate scraper;
use encoding::{Encoding, DecoderTrap};
use encoding::all::ISO_8859_1;
use std::io::{self, Read, Write};
use std::time::{Duration, SystemTime};
extern crate time;
use std::fs::File;
use std::path::Path;
extern crate flexi_logger;
use std::io::Seek;
use std::io::SeekFrom;
#[macro_use]
extern crate log;
use std::thread;
use teleborg::{Dispatcher, Bot, Updater};
use teleborg::objects::Update;
use flexi_logger::Logger;
use scraper::{Selector, Html};
use std::fs::OpenOptions;
use std::env;


fn remove_spaces_and_tabs(selector: &Selector, fragment: &Html) -> String {
    let mut formated = fragment.select(&selector).next().unwrap().text().collect::<String>();
    formated = formated.replace("\n","").replace("\t","");
    formated.push('\n');
    info!("Outputing: {:?}", formated);
    return formated;
}


fn get_site_response() -> reqwest::Result<reqwest::Response>{
    reqwest::get("https://www.prefeitura.unicamp.br/apps/site/cardapio.php")
}

fn filter_response(resp: &mut reqwest::Response) -> std::result::Result<String, String> {
    let mut body = Vec::new();

    let mut body_str = String::new();


    match resp.read_to_end(&mut body) {
        Ok(_) => {
            body_str = ISO_8859_1.decode(&*body, DecoderTrap::Strict).unwrap();
        },
        Err(e) => {
            return Err(format!("Error reading site response {}", e));
        }
    }
    let fragment = Html::parse_fragment(&body_str);


    let selector_breakfast = Selector::parse("#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(2)").unwrap();
    let selector_lunch_1 = Selector::parse("#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td:nth-child(1) > table > tbody > tr:nth-child(1) > td").unwrap();
    let selector_lunch_2 = Selector::parse("#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td:nth-child(1) > table > tbody > tr:nth-child(2) > td").unwrap();
    let selector_lunch_3 = Selector::parse("#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td:nth-child(1) > table > tbody > tr:nth-child(3) > td").unwrap();
    let selector_lunch_4 = Selector::parse("#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td").unwrap();
    let selector_lunch_5 = Selector::parse("#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td:nth-child(1) > table > tbody > tr:nth-child(5) > td").unwrap();
    let selector_lunch_6 = Selector::parse("#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td:nth-child(1) > table > tbody > tr:nth-child(6) > td").unwrap();

    let selector_veg_lunch_1 = Selector::parse("#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td:nth-child(2) > table > tbody > tr:nth-child(1) > td").unwrap();
    let selector_veg_lunch_2 = Selector::parse("#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td:nth-child(2) > table > tbody > tr:nth-child(2) > td").unwrap();
    let selector_veg_lunch_4 = Selector::parse("#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td:nth-child(2) > table > tbody > tr:nth-child(4) > td").unwrap();
    let selector_veg_lunch_5 = Selector::parse("#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td:nth-child(2) > table > tbody > tr:nth-child(5) > td").unwrap();
    let selector_veg_lunch_6 = Selector::parse("#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td:nth-child(2) > table > tbody > tr:nth-child(6) > td").unwrap();


    let selector_dinner_1 = Selector::parse("#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td:nth-child(3) > table > tbody > tr:nth-child(1) > td").unwrap();
    let selector_dinner_2 = Selector::parse("#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td:nth-child(3) > table > tbody > tr:nth-child(2) > td").unwrap();
    let selector_dinner_3 = Selector::parse("#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td:nth-child(3) > table > tbody > tr:nth-child(3) > td").unwrap();
    let selector_dinner_4 = Selector::parse("#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td:nth-child(3) > table > tbody > tr:nth-child(4) > td").unwrap();
    let selector_dinner_5 = Selector::parse("#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td:nth-child(3) > table > tbody > tr:nth-child(5) > td").unwrap();
    let selector_dinner_6 = Selector::parse("#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td:nth-child(3) > table > tbody > tr:nth-child(6) > td").unwrap();

    let selector_veg_dinner_1 = Selector::parse("#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td:nth-child(4) > table > tbody > tr:nth-child(1) > td").unwrap();
    let selector_veg_dinner_2 = Selector::parse("#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td:nth-child(4) > table > tbody > tr:nth-child(2) > td").unwrap();
    let selector_veg_dinner_4 = Selector::parse("#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td:nth-child(4) > table > tbody > tr:nth-child(4) > td").unwrap();
    let selector_veg_dinner_5 = Selector::parse("#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td:nth-child(4) > table > tbody > tr:nth-child(5) > td").unwrap();
    let selector_veg_dinner_6 = Selector::parse("#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td:nth-child(4) > table > tbody > tr:nth-child(6) > td").unwrap();

    let mut cafe_da_manha = String::from("Café da Manha: \n");
    cafe_da_manha.push_str(remove_spaces_and_tabs(&selector_breakfast, &fragment).as_str());


    let mut lunch = String::from("Almoço: \n");
    lunch.push_str(remove_spaces_and_tabs(&selector_lunch_1, &fragment).as_str());
    lunch.push_str(remove_spaces_and_tabs(&selector_lunch_2, &fragment).as_str());
    lunch.push_str(remove_spaces_and_tabs(&selector_lunch_3, &fragment).as_str());
    lunch.push_str(remove_spaces_and_tabs(&selector_lunch_4, &fragment).as_str());
    lunch.push_str(remove_spaces_and_tabs(&selector_lunch_5, &fragment).as_str());
    lunch.push_str(remove_spaces_and_tabs(&selector_lunch_6, &fragment).as_str());


    let mut lunch_veg = String::from("Almoço Vegetariano: \n");
    lunch_veg.push_str(remove_spaces_and_tabs(&selector_veg_lunch_1, &fragment).as_str());
    lunch_veg.push_str(remove_spaces_and_tabs(&selector_veg_lunch_2, &fragment).as_str());
    lunch_veg.push_str(remove_spaces_and_tabs(&selector_veg_lunch_4, &fragment).as_str());
    lunch_veg.push_str(remove_spaces_and_tabs(&selector_veg_lunch_5, &fragment).as_str());
    lunch_veg.push_str(remove_spaces_and_tabs(&selector_veg_lunch_6, &fragment).as_str());



    let mut dinner = String::from("Jantar: \n");
    dinner.push_str(remove_spaces_and_tabs(&selector_dinner_1, &fragment).as_str());
    dinner.push_str(remove_spaces_and_tabs(&selector_dinner_2, &fragment).as_str());
    dinner.push_str(remove_spaces_and_tabs(&selector_dinner_3, &fragment).as_str());
    dinner.push_str(remove_spaces_and_tabs(&selector_dinner_4, &fragment).as_str());
    dinner.push_str(remove_spaces_and_tabs(&selector_dinner_5, &fragment).as_str());
    dinner.push_str(remove_spaces_and_tabs(&selector_dinner_6, &fragment).as_str());


    let mut dinner_veg = String::from("Jantar Vegetariano: \n");
    dinner_veg.push_str(remove_spaces_and_tabs(&selector_veg_dinner_1, &fragment).as_str());
    dinner_veg.push_str(remove_spaces_and_tabs(&selector_veg_dinner_2, &fragment).as_str());
    dinner_veg.push_str(remove_spaces_and_tabs(&selector_veg_dinner_4, &fragment).as_str());
    dinner_veg.push_str(remove_spaces_and_tabs(&selector_veg_dinner_5, &fragment).as_str());
    dinner_veg.push_str(remove_spaces_and_tabs(&selector_veg_dinner_6, &fragment).as_str());


    return Ok(format!("{}\n{}\n{}\n{}", lunch, dinner, lunch_veg, dinner_veg));

}
//
fn get_menu() -> std::result::Result<String, ()>{
    let mut resp =
        match get_site_response(){
            Ok(response) => response,
            Err(e) => {
                error!("Could not get site error: {}", e);
                return Err(());
            }
        };
    return match filter_response(&mut resp){
        Ok(menu) => Ok(menu),
        Err(e) => Err(())
    }

}
//
fn has_already_published_today(file: &mut File) -> bool {
    let today_as_number = time::now().tm_mday.to_string();
    let mut day_last_broadcast = String::new();

    file.read_to_string(&mut day_last_broadcast);

    if day_last_broadcast == today_as_number {
        return true;
    }else {
        return false;
    }
}
//
fn broadcast_if_needed(bot: &Bot, path: &Path, rep_chat_id: &i64){


    let mut file =  OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .open(&path)
        .unwrap();

    if has_already_published_today(&mut file){
        return;
    }else {
        info!("Time to broadcast!");
        let menu = get_menu();
        match menu {
            Ok(menu) => {
                broadcast(&bot, menu, &rep_chat_id);
                info!("Broadcast successful");
            },
            Err(e) => {
                error!("Broadcast failed");
                broadcast(&bot,
                          "O site da prefeitura (https://www.prefeitura.unicamp.br/apps/site/cardapio.php) parece estar offline (ou a internet da rep caiu). Não pude pegar o cardápio. Tentarei de novo em 6 horas".to_string(),
                          &rep_chat_id);
                error!("Sleeping for 6 hours");
                std::thread::sleep(    std::time::Duration::from_secs(60*60*6));
                return;
            }
        }
        let mut file = File::create(path).unwrap();
        file.write(time::now().tm_mday.to_string().as_bytes());
    }
}

fn broadcast(bot: &Bot, message: String, rep_chat_id: &i64){
    let result = bot.send_message(
        rep_chat_id,
        &message,
        None,
        None,
        None,
        None,
        None,
    );
    match result {
        Ok(response) => info!("Successfully broadcast \n{:?}.", response),
        Err(e) => error!("Failed to broadcast, got \n{:?} as response.", e)
    }
}

fn main(){
    Logger::with_str("info")
        .log_to_file()
        .directory("log_files")
        .start()
        .unwrap_or_else(|e| panic!("Logger initialization failed with {}", e));

    let bot_token = env::var("TELEGRAM_BOT_ID")
        .ok()
        .expect("Can't find TELEGRAM_BOT_ID env variable")
        .parse::<String>()
        .unwrap();

    let rep_chat_id = env::var("REP_CHAT_ID")
        .ok()
        .expect("Can't find REP_CHAT_ID env variable")
        .parse::<i64>()
        .unwrap();


    let two_sec = std::time::Duration::from_secs(2);
    let path = Path::new("last_day_when_broadcasted.txt");
    let mut dispatcher = Dispatcher::new();
    dispatcher.add_command_handler("bandejao", send_menu, false);
    dispatcher.add_command_handler("bandeco", send_menu, false);
    dispatcher.add_command_handler("ru", send_menu, false);

    const BASE_URL: &'static str = "https://api.telegram.org/bot";

    let bot_url = [BASE_URL, &bot_token].concat();

    let bot = Bot::new(bot_url).unwrap();

    thread::spawn(move || {
        while true {
            broadcast_if_needed(&bot, &path, &rep_chat_id);
            std::thread::sleep(two_sec);
        }
    });

    Updater::start(Some(bot_token.clone()), None, None, None, dispatcher);

}

fn send_menu(bot: &Bot, update: Update, _: Option<Vec<&str>>) {
    info!("Got request for menu! {:?}", update);
    info!("Replying to it");
    let menu = get_menu().unwrap();
    let response = bot.reply_to_message(&update, menu.as_str());
    match response {
        Ok(response) => info!("Successfully sent \n{:?}.", response),
        Err(e) => error!("Failed to send \n{}, got \n{:?} as response.", menu, e)
    }
}