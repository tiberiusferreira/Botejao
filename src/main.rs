use unicamp_menu::*;

use chrono::prelude::*;
use teloxide::Bot;
use teloxide::types::{ChatId, ParseMode};
use teloxide::requests::Request;
use std::sync::Arc;
use std::time::Duration;
use flexi_logger::{colored_opt_format};


#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    flexi_logger::Logger::with_str("trace")
        .format(colored_opt_format)
        .log_to_file()
        .directory("./logs")
        .start().unwrap();

    let bot = Bot::from_env();
    let time_between_updates = 60*60;
    let mut old_menu;
    log::info!("Initializing");
    loop{
        match get_menu().await{
            Ok(menu) => {
                old_menu = menu;
                break;
            },
            Err(err) => {
                log::info!("Error initializing: {}.\n Trying again in {}s", err, time_between_updates);
                async_std::task::sleep(Duration::from_secs(time_between_updates)).await;
            },
        }
    }
    log::info!("Initialized!");

    loop{
        if let Err(error) = send_new_menu_if_updated(&bot, &mut old_menu).await{
            log::error!("Error: {}.\n Sleeping 60s", error.to_string());
            async_std::task::sleep(Duration::from_secs(60)).await;
        }
        log::debug!("Checked for updates. Sleeping for {}s", time_between_updates);
        async_std::task::sleep(Duration::from_secs(time_between_updates)).await;
    }

}


pub async fn send_new_menu_if_updated(bot: &Arc<Bot>, old_menu: &mut StructuredWeekMenus) -> Result<(), BotejaoError>{
    let mut new_menu = get_menu().await?;
    new_menu.next_menus.reverse();
    if &mut new_menu == old_menu {
        return Ok(());
    }
    let formatted_menu = format_week_menu(&new_menu);
    *old_menu = new_menu;
    send_new_menu(&bot, formatted_menu).await?;
    Ok(())
}



pub async fn send_new_menu(bot: &Arc<Bot>, formatted_menu: String) -> Result<(), BotejaoError>{
    let menu_msg = bot
        .send_message(
            ChatId::ChannelUsername("@botejaotest".to_string()),
            formatted_menu,
        )
        .parse_mode(ParseMode::HTML);
    menu_msg.send().await.map_err(|e|{
        BotejaoError::NetworkError(e.to_string())
    })?;
    Ok(())
}

pub fn format_week_menu(menu: &StructuredWeekMenus) -> String{
    let mut formatted_menu = String::new();

    // TODO: Put Camera URL when it comes back online
    //<a href=\"https://webservices.prefeitura.unicamp.br/cameras/cam_ra.jpg\">Câmera RA</a>
    formatted_menu.push_str("<a href=\"https://www.prefeitura.unicamp.br/servicos/divisao-de-alimentacao/cardapio-dos-restaurantes\">Cardapio Online</a>\n\n");

    for menu in &menu.next_menus{
        formatted_menu.push_str(&format_day_menu(&menu));
        formatted_menu.push_str("\n");
    }
    formatted_menu
}

pub fn format_day_menu(day_menu: &StructuredDayMenu) -> String{
    let pt_day = weekday_to_portuguese(day_menu.day.weekday());
    let mut title = format!("******* <b>{} {}</b> ******* \n", pt_day.to_uppercase(), day_menu.day.format("%d/%m/%Y"));
    if let Some(menu) = &day_menu.lunch{
        title.push_str(&format_cardapio(menu, MealKind::Lunch));
        title.push_str("\n");
    }
    if let Some(menu) = &day_menu.dinner{
        title.push_str(&format_cardapio(menu, MealKind::Dinner));
        title.push_str("\n");
    }
    if let Some(menu) = &day_menu.veg_lunch{
        title.push_str(&format_cardapio(menu, MealKind::VegLunch));
        title.push_str("\n");
    }
    if let Some(menu) = &day_menu.veg_dinner{
        title.push_str(&format_cardapio(menu, MealKind::VegDinner));
        title.push_str("\n");
    }
    title
}

pub fn format_cardapio(menu: &Cardapio, meal_kind: MealKind) -> String{
    let main_course_emoji;
    match meal_kind{
        MealKind::Lunch => {
            main_course_emoji = "🍗";
        },
        MealKind::Dinner => {
            main_course_emoji = "🍖";
        },
        MealKind::VegLunch => {
            main_course_emoji = "🥕";
        },
        MealKind::VegDinner => {
            main_course_emoji = "🥒";
        },
    }

    let mut buffer = String::new();
    buffer.push_str(&format!("🍲 <b>{}</b>:\n", menu.tipo));
    buffer.push_str(&format!("{} {}\n", main_course_emoji, uppercase_first_letter(&menu.prato_principal.to_lowercase())));
    if menu.pts.to_lowercase() != "não informado" {
        buffer.push_str(&format!("🌾 {}\n", uppercase_first_letter(&menu.pts.to_lowercase())));
    }
    buffer.push_str(&format!("🥬 {}\n", uppercase_first_letter(&menu.salada.to_lowercase())));
    buffer.push_str(&format!("🍦 {}\n", uppercase_first_letter(&menu.sobremesa.to_lowercase())));
    buffer.push_str(&format!("🧃 Suco de {}\n", menu.suco.to_lowercase()));
    buffer
}

fn uppercase_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

pub fn weekday_to_portuguese(day: Weekday) -> String{
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