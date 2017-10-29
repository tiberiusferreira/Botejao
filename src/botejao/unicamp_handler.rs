use teleborg::{Bot, Command, ParseMode, Updater};
use teleborg::objects::{Update, Message};
use teleborg::error::Error;
use reqwest;
use scraper::{Html, Selector};
use encoding::{DecoderTrap, Encoding};
use encoding::all::ISO_8859_1;
use std::io::{Read, Write};

pub struct UnicampHandler {
    menu_website: String
}

impl Command for UnicampHandler {
    fn execute(&mut self, bot: &Bot, update: Update, args: Option<Vec<&str>>) {
        self.send_unicamp_menu(bot, update, args);
    }
}

impl UnicampHandler {

    pub fn new() -> UnicampHandler {
        UnicampHandler{
            menu_website: "https://www.prefeitura.unicamp.br/apps/site/cardapio.php".to_string()
        }
    }

    pub fn send_unicamp_menu(&self, bot: &Bot, update: Update, _: Option<Vec<&str>>) {
        info!("Got request for UNICAMPs menu from user: {}", &update.message.as_ref().unwrap().from.as_ref().unwrap().username.as_ref().unwrap());
        info!("Replying to it");
        let menu = self.get_unicamp_menu().unwrap();
        let response = UnicampHandler::reply_to_message_as_markdown(&bot, &update, menu.as_str());
        match response {
            Ok(response) => info!("Successfully sent UNICAMPs menu."),
            Err(e) => error!("Failed to send \n{}, got \n{:?} as response.", menu, e),
        }
    }

    fn get_unicamp_menu(&self) -> Result<String, ()> {
        let mut resp = match reqwest::get(self.menu_website.as_str()) {
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

    pub fn reply_to_message_as_markdown(
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

    fn remove_spaces_and_tabs(input: String) -> String {
        let mut input = input.replace("\n", "").replace("\t", "");
        input.push('\n');
        return input;
    }

    fn apply_selectors(selector: &Selector, fragment: &Html) -> String {
        fragment
            .select(&selector)
            .next()
            .unwrap()
            .text()
            .collect::<String>()
    }



    fn filter_unicamp_response(
        resp: &mut reqwest::Response
    ) -> Result<String, String> {
        let mut body = Vec::new();
        let body_str;



        match resp.read_to_end(&mut body) {
            Ok(_) => {
                body_str = ISO_8859_1.decode(&*body, DecoderTrap::Strict).unwrap();
            }
            Err(e) => {
                return Err(format!("Error reading site response {}", e));
            }
        }
        let fragment = Html::parse_fragment(&body_str);

        let day_selector = Selector::parse(
            "#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > p",
        ).unwrap();

        let selector_breakfast = Selector::parse(
            "#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(2)",
        ).unwrap();


        let mut lunch_selectors = Vec::new();
        lunch_selectors.push(
            Selector::parse(
                "#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td:nth-child(1) > table > tbody > tr:nth-child(1) > td",
            ).unwrap(),
        );
        lunch_selectors.push(
            Selector::parse(
                "#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td:nth-child(1) > table > tbody > tr:nth-child(2) > td",
            ).unwrap(),
        );
        lunch_selectors.push(
            Selector::parse(
                "#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td:nth-child(1) > table > tbody > tr:nth-child(3) > td",
            ).unwrap(),
        );
        lunch_selectors.push(
            Selector::parse(
                "#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td",
            ).unwrap(),
        );
        lunch_selectors.push(
            Selector::parse(
                "#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td:nth-child(1) > table > tbody > tr:nth-child(5) > td",
            ).unwrap(),
        );
        lunch_selectors.push(
            Selector::parse(
                "#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td:nth-child(1) > table > tbody > tr:nth-child(6) > td",
            ).unwrap(),
        );

        let mut veg_lunch_selectors = Vec::new();

        veg_lunch_selectors.push(
            Selector::parse(
                "#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td:nth-child(2) > table > tbody > tr:nth-child(1) > td",
            ).unwrap(),
        );
        veg_lunch_selectors.push(
            Selector::parse(
                "#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td:nth-child(2) > table > tbody > tr:nth-child(2) > td",
            ).unwrap(),
        );
        veg_lunch_selectors.push(
            Selector::parse(
                "#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td:nth-child(2) > table > tbody > tr:nth-child(4) > td",
            ).unwrap(),
        );
        veg_lunch_selectors.push(
            Selector::parse(
                "#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td:nth-child(2) > table > tbody > tr:nth-child(5) > td",
            ).unwrap(),
        );
        veg_lunch_selectors.push(
            Selector::parse(
                "#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td:nth-child(2) > table > tbody > tr:nth-child(6) > td",
            ).unwrap(),
        );

        let mut dinner_selectors = Vec::new();

        dinner_selectors.push(
            Selector::parse(
                "#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td:nth-child(3) > table > tbody > tr:nth-child(1) > td",
            ).unwrap(),
        );
        dinner_selectors.push(
            Selector::parse(
                "#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td:nth-child(3) > table > tbody > tr:nth-child(2) > td",
            ).unwrap(),
        );
        dinner_selectors.push(
            Selector::parse(
                "#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td:nth-child(3) > table > tbody > tr:nth-child(3) > td",
            ).unwrap(),
        );
        dinner_selectors.push(
            Selector::parse(
                "#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td:nth-child(3) > table > tbody > tr:nth-child(4) > td",
            ).unwrap(),
        );
        dinner_selectors.push(
            Selector::parse(
                "#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td:nth-child(3) > table > tbody > tr:nth-child(5) > td",
            ).unwrap(),
        );
        dinner_selectors.push(
            Selector::parse(
                "#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td:nth-child(3) > table > tbody > tr:nth-child(6) > td",
            ).unwrap(),
        );

        let mut veg_dinner_selectors = Vec::new();
        veg_dinner_selectors.push(
            Selector::parse(
                "#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td:nth-child(4) > table > tbody > tr:nth-child(1) > td",
            ).unwrap(),
        );
        veg_dinner_selectors.push(
            Selector::parse(
                "#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td:nth-child(4) > table > tbody > tr:nth-child(2) > td",
            ).unwrap(),
        );
        veg_dinner_selectors.push(
            Selector::parse(
                "#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td:nth-child(4) > table > tbody > tr:nth-child(4) > td",
            ).unwrap(),
        );
        veg_dinner_selectors.push(
            Selector::parse(
                "#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td:nth-child(4) > table > tbody > tr:nth-child(5) > td",
            ).unwrap(),
        );
        veg_dinner_selectors.push(
            Selector::parse(
                "#sistema_cardapio > table > tbody > tr:nth-child(1) > td:nth-child(1) > table > tbody > tr:nth-child(4) > td:nth-child(4) > table > tbody > tr:nth-child(6) > td",
            ).unwrap(),
        );

        let mut cafe_da_manha = String::from("*Café da Manha:* \n");

        let extracted_break_fast = UnicampHandler::apply_selectors(&selector_breakfast, &fragment);

        cafe_da_manha.push_str(UnicampHandler::remove_spaces_and_tabs(extracted_break_fast).as_str());

        let extracted_day = UnicampHandler::apply_selectors(&day_selector, &fragment);

        let day = format!("*{}*", UnicampHandler::remove_spaces_and_tabs(extracted_day));


        let mut lunch = String::from("*Almoço:* \n");

        for selector in &lunch_selectors {
            lunch.push_str(
                UnicampHandler::remove_spaces_and_tabs(UnicampHandler::apply_selectors(selector, &fragment))
                    .as_str(),
            );
        }

        let mut lunch_veg = String::from("*Almoço Vegetariano:* \n");

        for selector in &veg_lunch_selectors {
            lunch_veg.push_str(
                UnicampHandler::remove_spaces_and_tabs(UnicampHandler::apply_selectors(selector, &fragment))
                    .as_str(),
            );
        }
        let mut dinner = String::from("*Jantar:* \n");

        for selector in &dinner_selectors {
            dinner.push_str(
                UnicampHandler::remove_spaces_and_tabs(UnicampHandler::apply_selectors(selector, &fragment))
                    .as_str(),
            );
        }

        let mut dinner_veg = String::from("*Jantar Vegetariano: *\n");

        for selector in &veg_dinner_selectors {
            dinner_veg.push_str(
                UnicampHandler::remove_spaces_and_tabs(UnicampHandler::apply_selectors(selector, &fragment))
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
