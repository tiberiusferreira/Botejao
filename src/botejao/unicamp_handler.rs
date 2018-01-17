use teleborg::{Bot, ParseMode};
use teleborg::objects::{Update, Message};
use teleborg::error::Error;
use reqwest;
use scraper::{Html, Selector};
use encoding::{DecoderTrap, Encoding};
use encoding::all::ISO_8859_1;
use std::io::{Read, Write};
use std::sync::{Arc, RwLock};
use std::error;
use std::thread;
use std::time::Duration;

const MENU_WEBSITE: &str = "https://www.prefeitura.unicamp.br/apps/site/cardapio.php";

pub struct UnicampHandler {
    cached_menu: Arc<RwLock<String>>
}

//impl Command for UnicampHandler {
//    fn execute(&mut self, bot: &Bot, update: Update, args: Option<Vec<&str>>) {
//        self.send_unicamp_menu(bot, update, args);
//    }
//}

impl UnicampHandler {

    pub fn new() -> UnicampHandler {

        let unicamp_handler = UnicampHandler{
            cached_menu: Arc::new(RwLock::new("Atualizando menu, tente de novo em alguns segundos.".to_string())),
        };
        let thread_menu_ref = unicamp_handler.cached_menu.clone();
        thread::spawn(move ||{
            loop {
                let new_menu = match UnicampHandler::get_fresh_unicamp_menu() {
                    Ok(new_menu) => new_menu,
                    Err(_) => continue
                };
                {
                    let mut cached_menu = thread_menu_ref.write().unwrap();
                    *cached_menu = new_menu;
                }
                thread::sleep(Duration::from_secs(10*60));
            }
        });
        unicamp_handler

    }


    pub fn get_unicamp_menu(&self) -> String{
        let menu = self.cached_menu.read().unwrap();
        menu.clone()
    }
    fn get_fresh_unicamp_menu() -> Result<String, ()> {
        let mut resp = match reqwest::get(MENU_WEBSITE) {
            Ok(response) => response,
            Err(e) => {
                error!("Could not get site error: {}", e);
                return Err(());
            }
        };
        return match UnicampHandler::filter_unicamp_response(&mut resp) {
            Ok(menu) => Ok(menu),
            Err(_) => Err(()),
        };
    }

    fn remove_spaces_and_tabs(input: String) -> String {
        let mut input = input.replace("\n", "").replace("\t", "");
        input.push('\n');
        return input;
    }

    fn apply_selectors(selector: &Selector, fragment: &Html) -> Result<String, ()> {
        let next = match fragment
            .select(&selector)
            .next(){
            Some(next) => next,
            None => return Err(())
        };
        let result = next.text().collect::<String>();
        Ok(result)
    }



    fn filter_unicamp_response(resp: &mut reqwest::Response) -> Result<String, ()> {
        let mut body = Vec::new();
        let body_str;



        match resp.read_to_end(&mut body){
            Ok(_) => {},
            Err(e) => return Err(()),
        }
        body_str = match ISO_8859_1.decode(&*body, DecoderTrap::Strict){
            Ok(res) => {res},
            Err(e) => return Err(()),
        };
        let fragment = Html::parse_fragment(&body_str);

        let day_selector = Selector::parse(
            "#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > p",
        )?;

        let selector_breakfast = Selector::parse(
            "#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(2)",
        )?;


        let mut lunch_selectors = Vec::new();
        lunch_selectors.push(
            Selector::parse(
                "#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td:nth-child(1) > table > tbody > tr:nth-child(1) > td",
            )?,
        );
        lunch_selectors.push(
            Selector::parse(
                "#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td:nth-child(1) > table > tbody > tr:nth-child(2) > td",
            )?,
        );
        lunch_selectors.push(
            Selector::parse(
                "#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td:nth-child(1) > table > tbody > tr:nth-child(3) > td",
            )?,
        );
        lunch_selectors.push(
            Selector::parse(
                "#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td",
            )?,
        );
        lunch_selectors.push(
            Selector::parse(
                "#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td:nth-child(1) > table > tbody > tr:nth-child(5) > td",
            )?,
        );
        lunch_selectors.push(
            Selector::parse(
                "#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td:nth-child(1) > table > tbody > tr:nth-child(6) > td",
            )?,
        );

        let mut veg_lunch_selectors = Vec::new();

        veg_lunch_selectors.push(
            Selector::parse(
                "#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td:nth-child(2) > table > tbody > tr:nth-child(1) > td",
            )?,
        );
        veg_lunch_selectors.push(
            Selector::parse(
                "#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td:nth-child(2) > table > tbody > tr:nth-child(2) > td",
            )?,
        );
        veg_lunch_selectors.push(
            Selector::parse(
                "#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td:nth-child(2) > table > tbody > tr:nth-child(4) > td",
            )?,
        );
        veg_lunch_selectors.push(
            Selector::parse(
                "#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td:nth-child(2) > table > tbody > tr:nth-child(5) > td",
            )?,
        );
        veg_lunch_selectors.push(
            Selector::parse(
                "#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td:nth-child(2) > table > tbody > tr:nth-child(6) > td",
            )?,
        );

        let mut dinner_selectors = Vec::new();

        dinner_selectors.push(
            Selector::parse(
                "#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td:nth-child(3) > table > tbody > tr:nth-child(1) > td",
            )?,
        );
        dinner_selectors.push(
            Selector::parse(
                "#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td:nth-child(3) > table > tbody > tr:nth-child(2) > td",
            )?,
        );
        dinner_selectors.push(
            Selector::parse(
                "#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td:nth-child(3) > table > tbody > tr:nth-child(3) > td",
            )?,
        );
        dinner_selectors.push(
            Selector::parse(
                "#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td:nth-child(3) > table > tbody > tr:nth-child(4) > td",
            )?,
        );
        dinner_selectors.push(
            Selector::parse(
                "#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td:nth-child(3) > table > tbody > tr:nth-child(5) > td",
            )?,
        );
        dinner_selectors.push(
            Selector::parse(
                "#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td:nth-child(3) > table > tbody > tr:nth-child(6) > td",
            )?,
        );

        let mut veg_dinner_selectors = Vec::new();
        veg_dinner_selectors.push(
            Selector::parse(
                "#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td:nth-child(4) > table > tbody > tr:nth-child(1) > td",
            )?,
        );
        veg_dinner_selectors.push(
            Selector::parse(
                "#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td:nth-child(4) > table > tbody > tr:nth-child(2) > td",
            )?,
        );
        veg_dinner_selectors.push(
            Selector::parse(
                "#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td:nth-child(4) > table > tbody > tr:nth-child(4) > td",
            )?,
        );
        veg_dinner_selectors.push(
            Selector::parse(
                "#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td:nth-child(4) > table > tbody > tr:nth-child(5) > td",
            )?,
        );
        veg_dinner_selectors.push(
            Selector::parse(
                "#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td:nth-child(4) > table > tbody > tr:nth-child(6) > td",
            )?,
        );

        let mut cafe_da_manha = String::from("*Café da Manha:* \n");

        let extracted_break_fast = UnicampHandler::apply_selectors(&selector_breakfast, &fragment)?;

        cafe_da_manha.push_str(UnicampHandler::remove_spaces_and_tabs(extracted_break_fast).as_str());

        let extracted_day = UnicampHandler::apply_selectors(&day_selector, &fragment)?;

        let day = format!("*{}*", UnicampHandler::remove_spaces_and_tabs(extracted_day));


        let mut lunch = String::from("*Almoço:* \n");

        for selector in &lunch_selectors {
            lunch.push_str(
                UnicampHandler::remove_spaces_and_tabs(UnicampHandler::apply_selectors(selector, &fragment)?)
                    .as_str(),
            );
        }

        let mut lunch_veg = String::from("*Almoço Vegetariano:* \n");

        for selector in &veg_lunch_selectors {
            lunch_veg.push_str(
                UnicampHandler::remove_spaces_and_tabs(UnicampHandler::apply_selectors(selector, &fragment)?)
                    .as_str(),
            );
        }
        let mut dinner = String::from("*Jantar:* \n");

        for selector in &dinner_selectors {
            dinner.push_str(
                UnicampHandler::remove_spaces_and_tabs(UnicampHandler::apply_selectors(selector, &fragment)?)
                    .as_str(),
            );
        }

        let mut dinner_veg = String::from("*Jantar Vegetariano: *\n");

        for selector in &veg_dinner_selectors {
            dinner_veg.push_str(
                UnicampHandler::remove_spaces_and_tabs(UnicampHandler::apply_selectors(selector, &fragment)?)
                    .as_str(),
            );
        }
        return Ok(format!(
            "{}\n {}\n{}\n{}\n{}",
            day,
            lunch,
            dinner,
            lunch_veg,
            dinner_veg
        ));
    }


}
