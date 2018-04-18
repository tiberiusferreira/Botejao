extern crate teleborg;
use reqwest;
use scraper::{Html, Selector};

use encoding::{DecoderTrap, Encoding};
use encoding::all::ISO_8859_1;
use std::io::{Read};
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;
use menu_comparator;
use teleborg::objects::OutgoingChannelMessage;
use teleborg::Bot;
use teleborg::TelegramInterface;
use failure::*;
use self::chrono::prelude::*;
use self::chrono::offset::LocalResult;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

extern crate chrono;

const MENU_WEBSITE: &str = "https://www.prefeitura.unicamp.br/apps/site/cardapio.php";

pub struct UnicampMenu{
    break_fast: String,
    lunch: String,
    veg_lunch: String,
    dinner: String,
    veg_dinner: String,
    menu_with_headers: String
}
pub struct UnicampHandler {
    cached_menu: Arc<RwLock<String>>,
}

#[derive(Fail, Debug)]
#[fail(display = "An error occurred.")]
struct MyError;



#[derive(Debug, Fail)]
#[fail(display = "Error while filtering the menu")]
pub struct MenuFilteringError {}

pub struct UnicampMenuGetter;

//impl From<ParseError> for MenuFilteringError {
//    fn from(parse_error: ParseError) -> Self {
//        MenuFilteringError{}
//    }
//}


impl UnicampMenuGetter{
    fn get_fresh_unicamp_menu() -> Result<UnicampMenu, ()> {
        let mut resp = match reqwest::get(MENU_WEBSITE) {
            Ok(response) => response,
            Err(e) => {
                error!("Could not get site error: {}", e);
                return Err(());
            }
        };

        info!("Got response");

        return match UnicampMenuGetter::filter_unicamp_response(&mut resp) {
            Ok(menu) => Ok(menu),
            Err(_) => Err(()),
        };
    }

    fn remove_spaces_and_tabs(input: String) -> String {
        let mut input = input.replace("\n", "").replace("\t", "");
        input.push('\n');
        return input;
    }

    fn apply_selectors(selector: &Selector, fragment: &Html) -> Result<String, MenuFilteringError> {
        let next = match fragment
            .select(&selector)
            .next(){
            Some(next) => next,
            None => return Err(MenuFilteringError{})
        };
        let result = next.text().collect::<String>();
        Ok(result)
    }



    fn parse_using_css(selector: &str) -> Result<Selector, MenuFilteringError>{
        Selector::parse(
            selector).map_err(|err|{
            MenuFilteringError{}
        })
    }


    fn filter_unicamp_response(resp: &mut reqwest::Response) -> Result<UnicampMenu, Error> {
        let mut body = Vec::new();
        let body_str;

        resp.read_to_end(&mut body)?;

        body_str = ISO_8859_1.decode(&*body, DecoderTrap::Strict).map_err(|err|{
            MenuFilteringError{}
        })?;

        let fragment = Html::parse_fragment(&body_str);



        let day_selector = Self::parse_using_css(
            "#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > p")?;

        let selector_breakfast = Self::parse_using_css(
            "#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(2)")?;


        let mut lunch_selectors = Vec::new();


        lunch_selectors.push(
            Self::parse_using_css(
                "#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td:nth-child(1) > table > tbody > tr:nth-child(1) > td")?
        );



        lunch_selectors.push(
            Self::parse_using_css(
                "#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td:nth-child(1) > table > tbody > tr:nth-child(2) > td",
            )?,
        );
        lunch_selectors.push(
            Self::parse_using_css(
                "#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td:nth-child(1) > table > tbody > tr:nth-child(3) > td",
            )?,
        );
        lunch_selectors.push(
            Self::parse_using_css(
                "#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td",
            )?,
        );
        lunch_selectors.push(
            Self::parse_using_css(
                "#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td:nth-child(1) > table > tbody > tr:nth-child(5) > td",
            )?,
        );
        lunch_selectors.push(
            Self::parse_using_css(
                "#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td:nth-child(1) > table > tbody > tr:nth-child(6) > td",
            )?,
        );

        let mut veg_lunch_selectors = Vec::new();

        veg_lunch_selectors.push(
            Self::parse_using_css(
                "#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td:nth-child(2) > table > tbody > tr:nth-child(1) > td",
            )?,
        );
        veg_lunch_selectors.push(
            Self::parse_using_css(
                "#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td:nth-child(2) > table > tbody > tr:nth-child(2) > td",
            )?,
        );
        veg_lunch_selectors.push(
            Self::parse_using_css(
                "#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td:nth-child(2) > table > tbody > tr:nth-child(4) > td",
            )?,
        );
        veg_lunch_selectors.push(
            Self::parse_using_css(
                "#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td:nth-child(2) > table > tbody > tr:nth-child(5) > td",
            )?,
        );
        veg_lunch_selectors.push(
            Self::parse_using_css(
                "#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td:nth-child(2) > table > tbody > tr:nth-child(6) > td",
            )?,
        );

        let mut dinner_selectors = Vec::new();

        dinner_selectors.push(
            Self::parse_using_css(
                "#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td:nth-child(3) > table > tbody > tr:nth-child(1) > td",
            )?,
        );
        dinner_selectors.push(
            Self::parse_using_css(
                "#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td:nth-child(3) > table > tbody > tr:nth-child(2) > td",
            )?,
        );
        dinner_selectors.push(
            Self::parse_using_css(
                "#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td:nth-child(3) > table > tbody > tr:nth-child(3) > td",
            )?,
        );
        dinner_selectors.push(
            Self::parse_using_css(
                "#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td:nth-child(3) > table > tbody > tr:nth-child(4) > td",
            )?,
        );
        dinner_selectors.push(
            Self::parse_using_css(
                "#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td:nth-child(3) > table > tbody > tr:nth-child(5) > td",
            )?,
        );
        dinner_selectors.push(
            Self::parse_using_css(
                "#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td:nth-child(3) > table > tbody > tr:nth-child(6) > td",
            )?,
        );

        let mut veg_dinner_selectors = Vec::new();
        veg_dinner_selectors.push(
            Self::parse_using_css(
                "#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td:nth-child(4) > table > tbody > tr:nth-child(1) > td",
            )?,
        );
        veg_dinner_selectors.push(
            Self::parse_using_css(
                "#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td:nth-child(4) > table > tbody > tr:nth-child(2) > td",
            )?,
        );
        veg_dinner_selectors.push(
            Self::parse_using_css(
                "#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td:nth-child(4) > table > tbody > tr:nth-child(4) > td",
            )?,
        );
        veg_dinner_selectors.push(
            Self::parse_using_css(
                "#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td:nth-child(4) > table > tbody > tr:nth-child(5) > td",
            )?,
        );
        veg_dinner_selectors.push(
            Self::parse_using_css(
                "#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td:nth-child(4) > table > tbody > tr:nth-child(6) > td",
            )?,
        );

        let mut breakfast = String::from("*Café da Manha:* \n");

        let extracted_break_fast = UnicampMenuGetter::apply_selectors(&selector_breakfast, &fragment)?;

        breakfast.push_str(UnicampMenuGetter::remove_spaces_and_tabs(extracted_break_fast).as_str());

        let extracted_day = UnicampMenuGetter::apply_selectors(&day_selector, &fragment)?;

        let day = format!("*{}*", UnicampMenuGetter::remove_spaces_and_tabs(extracted_day));


        let mut lunch = String::from("*Almoço:* \n");

        for selector in &lunch_selectors {
            lunch.push_str(
                UnicampMenuGetter::remove_spaces_and_tabs(UnicampMenuGetter::apply_selectors(selector, &fragment)?)
                    .as_str(),
            );
        }

        let mut lunch_veg = String::from("*Almoço Vegetariano:* \n");

        for selector in &veg_lunch_selectors {
            lunch_veg.push_str(
                UnicampMenuGetter::remove_spaces_and_tabs(UnicampMenuGetter::apply_selectors(selector, &fragment)?)
                    .as_str(),
            );
        }
        let mut dinner = String::from("*Jantar:* \n");

        for selector in &dinner_selectors {
            dinner.push_str(
                UnicampMenuGetter::remove_spaces_and_tabs(UnicampMenuGetter::apply_selectors(selector, &fragment)?)
                    .as_str(),
            );
        }

        let mut dinner_veg = String::from("*Jantar Vegetariano: *\n");

        for selector in &veg_dinner_selectors {
            dinner_veg.push_str(
                UnicampMenuGetter::remove_spaces_and_tabs(UnicampMenuGetter::apply_selectors(selector, &fragment)?)
                    .as_str(),
            );
        }

        let formated_menu = format!(
            "{}\n {}\n{}\n{}\n{}",
            day,
            lunch,
            dinner,
            lunch_veg,
            dinner_veg
        );


        let unicamp_menu = UnicampMenu{
            break_fast: breakfast,
            lunch,
            veg_lunch: lunch_veg,
            dinner,
            veg_dinner: dinner_veg,
            menu_with_headers: formated_menu,
        };
        return Ok(unicamp_menu);
    }
}

struct UnicampChannelHandler{
    telegram_api: Bot,
}

impl UnicampChannelHandler{
    pub fn new(bot_token: String) -> UnicampChannelHandler {
        UnicampChannelHandler{
            telegram_api: Bot::new(bot_token).unwrap()
        }
    }

    pub fn handle_new_menu(&self, new_menu: String, previous_menu: String){

    }
}
impl UnicampHandler {

    pub fn new(bot_token: String) -> UnicampHandler {
        let unicamp_handler = UnicampHandler{
            cached_menu: Arc::new(RwLock::new("Atualizando menu, tente de novo em alguns segundos.".to_string())),
        };//        let thread_menu_ref = unicamp_handler.cached_menu.clone();
//        info!("Getting new menu!");

        thread::spawn(move ||{
            let unicamp_channel_handler = UnicampChannelHandler::new(bot_token.clone());

            loop {

                let new_menu = match UnicampMenuGetter::get_fresh_unicamp_menu() {
                    Ok(new_menu) => new_menu,
                    Err(_) => {
                        error!("Error getting UNICAMP menu!");
                        thread::sleep(Duration::from_secs(30));
                        continue
                    }
                };
                let dt = Local::now();

                if  dt.weekday() == Weekday::Sun || dt.weekday() == Weekday::Mon{
                    continue
                }


                if dt.hour() > 1 && Self::is_new_day(){
                    info!("New  day!");
                    Self::send_update_to_channel(&unicamp_channel_handler.telegram_api, new_menu.menu_with_headers);
                    Self::erase_and_create_new_day_file(dt.num_days_from_ce());
                }

                thread::sleep(Duration::from_secs(60*60));
            }
        });
        unicamp_handler
    }

    fn erase_and_create_new_day_file(current_day: i32){
        let mut file = File::create("current_day.txt").unwrap();
        file.write(format!("{}",current_day).as_bytes()).unwrap();
        file.sync_all().unwrap();
    }

    fn read_last_day_from_file() -> Result<i32, Error>{
        let file = File::open("current_day.txt")?;
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents)?;
        Ok(contents.parse::<i32>()?)
    }

    fn is_new_day() -> bool{
        let dt = Local::now();
        let current_day = dt.num_days_from_ce();
        let last_day;
        match Self::read_last_day_from_file(){
            Ok(day) => last_day = day,
            Err(_)=> {
                return true;
            }
        }
        if last_day == current_day{
            false
        }else {
            true
        }
    }
    fn send_update_to_channel(telegram_api: &TelegramInterface, new_menu: String){
        let outgoing_channel_message = OutgoingChannelMessage::new("@botejao_unicamp".to_string(),&new_menu);
        telegram_api.send_channel_msg(outgoing_channel_message);
    }


    pub fn get_unicamp_menu(&self) -> String{
        let menu = self.cached_menu.read().unwrap();
        menu.clone()
    }



}
