#![allow(warnings)] // remove when error_chain is fixed
extern crate telegram_bot;
extern crate tokio_core;
extern crate futures;
// botejão

extern crate reqwest;
#[macro_use]
extern crate error_chain;
extern crate scraper;
use telegram_bot::*;

use std::io::{self, Read, Write};

use scraper::{Selector, Html};
error_chain! {
    foreign_links {
        ReqError(reqwest::Error);
        IoError(std::io::Error);
    }
}


fn format_str(selector: &Selector, fragment: &Html) -> String {
    let mut formated = fragment.select(&selector).next().unwrap().text().collect::<String>();
    formated = formated.replace("\n","").replace("\t","");
    formated.push('\n');
    println!("Outputing: {:?}", formated);
    return formated;
}

fn run() -> Result<()> {

    let api = Api::from_token("454527929:AAHj82aCosGe1M8H6Wvohy0jznpkXLsjPq4").unwrap();
    let mut resp = reqwest::get("https://www.prefeitura.unicamp.br/apps/site/cardapio.php")?;

    let mut body = Vec::new();
    let mut body_str = String::new();


    resp.read_to_end(&mut body).unwrap();
    match resp.read_to_end(&mut body) {
        Ok(_) => body_str = String::from_utf8_lossy(&*body).to_string(),
        Err(why) => panic!("String conversion failure: {:?}", why),
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
    cafe_da_manha.push_str(format_str(&selector_breakfast, &fragment).as_str());


    let mut lunch = String::from("Almoço: \n");
    lunch.push_str(format_str(&selector_lunch_1, &fragment).as_str());
    lunch.push_str(format_str(&selector_lunch_2, &fragment).as_str());
    lunch.push_str(format_str(&selector_lunch_3, &fragment).as_str());
    lunch.push_str(format_str(&selector_lunch_4, &fragment).as_str());
    lunch.push_str(format_str(&selector_lunch_5, &fragment).as_str());
    lunch.push_str(format_str(&selector_lunch_6, &fragment).as_str());


    let mut lunch_veg = String::from("Almoço Vegetariano: \n");
    lunch_veg.push_str(format_str(&selector_veg_lunch_1, &fragment).as_str());
    lunch_veg.push_str(format_str(&selector_veg_lunch_2, &fragment).as_str());
    lunch_veg.push_str(format_str(&selector_veg_lunch_4, &fragment).as_str());
    lunch_veg.push_str(format_str(&selector_veg_lunch_5, &fragment).as_str());
    lunch_veg.push_str(format_str(&selector_veg_lunch_6, &fragment).as_str());



    let mut dinner = String::from("Jantar: \n");
    dinner.push_str(format_str(&selector_dinner_1, &fragment).as_str());
    dinner.push_str(format_str(&selector_dinner_2, &fragment).as_str());
    dinner.push_str(format_str(&selector_dinner_3, &fragment).as_str());
    dinner.push_str(format_str(&selector_dinner_4, &fragment).as_str());
    dinner.push_str(format_str(&selector_dinner_5, &fragment).as_str());
    dinner.push_str(format_str(&selector_dinner_6, &fragment).as_str());


    let mut dinner_veg = String::from("Jantar Vegetariano: \n");
    dinner_veg.push_str(format_str(&selector_veg_dinner_1, &fragment).as_str());
    dinner_veg.push_str(format_str(&selector_veg_dinner_2, &fragment).as_str());
    dinner_veg.push_str(format_str(&selector_veg_dinner_4, &fragment).as_str());
    dinner_veg.push_str(format_str(&selector_veg_dinner_5, &fragment).as_str());
    dinner_veg.push_str(format_str(&selector_veg_dinner_6, &fragment).as_str());


    let all_meals = format!("{}\n{}\n{}\n{}", lunch, dinner, lunch_veg, dinner_veg);
    println!("{:?}", dinner_veg);
        api.send_message(
            -1001121845452,
            all_meals,
            None,
            None,
            None,
            None,
        ).unwrap();



    Ok(())
}

quick_main!(run);