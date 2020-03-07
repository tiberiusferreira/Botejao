use unicamp_menu::*;

use chrono::prelude::*;
use chrono::ParseError;
use teloxide::Bot;
use teloxide::types::{ChatId, ParseMode};
use std::fmt::Write;
use teloxide::requests::Request;

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    flexi_logger::Logger::with_str("info").start().unwrap();

    let bot = Bot::from_env();


    let mut menu = get_menu().await.unwrap();
    let mut formatted_menu = String::new();
    formatted_menu.push_str("<a href=\"https://www.prefeitura.unicamp.br/servicos/divisao-de-alimentacao/cardapio-dos-restaurantes\">Cardapio Online</a>\n\n");
    for menu in &menu.next_menus{
        formatted_menu.push_str(&format_day_menu(&menu));
        formatted_menu.push_str("\n");
    }

    println!("{}", formatted_menu);


    let menu_msg = bot
        .send_message(
            ChatId::ChannelUsername("@botejaotest".to_string()),
            formatted_menu,
        )
        .parse_mode(ParseMode::HTML);

    menu_msg.send().await.unwrap();

}

pub fn format_day_menu(day_menu: &StructuredDayMenu) -> String{
    let pt_day = weekday_to_portuguese(&day_menu.day.weekday());
    let mut title = format!("**** <b>{}</b> **** {}\n", pt_day.to_uppercase(), day_menu.day.format("%d/%m/%Y"));
    if let Some(menu) = &day_menu.lunch{
        title.push_str(&format_cardapio(menu));
        title.push_str("\n");
    }
    if let Some(menu) = &day_menu.dinner{
        title.push_str(&format_cardapio(menu));
        title.push_str("\n");
    }
    if let Some(menu) = &day_menu.veg_lunch{
        title.push_str(&format_cardapio(menu));
        title.push_str("\n");
    }
    if let Some(menu) = &day_menu.veg_dinner{
        title.push_str(&format_cardapio(menu));
        title.push_str("\n");
    }
    title
}

pub fn format_cardapio(menu: &Cardapio) -> String{
    let mut buffer = String::new();
    buffer.push_str(&format!("<b>{}</b>:\n", menu.tipo));
    buffer.push_str(&format!("• {}\n", uppercase_first_letter(&menu.prato_principal.to_lowercase())));
    if menu.pts.to_lowercase() != "não informado" {
        buffer.push_str(&format!("• {}\n", uppercase_first_letter(&menu.pts.to_lowercase())));
    }
    buffer.push_str(&format!("• {}\n", uppercase_first_letter(&menu.salada.to_lowercase())));
    buffer.push_str(&format!("• {}\n", uppercase_first_letter(&menu.sobremesa.to_lowercase())));
    buffer.push_str(&format!("• Suco de {}\n", menu.suco.to_lowercase()));
    buffer
}

fn uppercase_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

pub fn weekday_to_portuguese(day: &Weekday) -> String{
    match day{
        Weekday::Mon => {"Segunda"},
        Weekday::Tue => {"Terça"},
        Weekday::Wed => {"Quarta"},
        Weekday::Thu => {"Quinta"},
        Weekday::Fri => {"Sexta"},
        Weekday::Sat => {"Sábado"},
        Weekday::Sun => {"Domingo"},
    }.to_string()
}