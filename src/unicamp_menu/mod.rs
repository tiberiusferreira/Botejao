mod error;
pub use error::BotejaoError;
use crate::unicamp_menu::structs::StructuredWeekMenus;
mod menu_structurer;
pub mod structs;


pub async fn get_menu() -> Result<StructuredWeekMenus, BotejaoError>{
    use crate::unicamp_menu::structs::WeekMenu;
    let mut res = surf::get("https://webservices.prefeitura.unicamp.br/cardapio_json.php")
        .await
        .map_err(|e| BotejaoError::NetworkError(e.to_string()))?;
    let menu: WeekMenu = res.body_json().await.map_err(|e| BotejaoError::NetworkError(e.to_string()))?;
    let structured = StructuredWeekMenus::from_week_menu(menu)?;
    Ok(structured)
}
