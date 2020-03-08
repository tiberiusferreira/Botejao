mod error;
pub use error::BotejaoError;
use crate::unicamp_menu::structs::StructuredWeekMenus;
mod menu_structurer;
pub mod structs;


pub async fn get_menu() -> Result<StructuredWeekMenus, BotejaoError>{
    get_menu_from_url("https://webservices.prefeitura.unicamp.br/cardapio_json.php").await
}


/// Attemps to download the menu from given URL.
///
/// # Examples
///
/// ```
/// #[tokio::main]
/// async fn main() {
/// let res = unicamp_menu::get_menu_from_url("https://webservices.prefeitura.unicamp.br/cardapio_json.php").await;
/// assert!(res.is_ok());
/// assert!(!res.unwrap().next_menus.is_empty());
/// }
/// ```
///
/// ```
/// #[tokio::main]
/// async fn main() {
///     use unicamp_menu::BotejaoError;
///     let res = unicamp_menu::get_menu_from_url("https://httpbin.org/json").await;
///     assert!(res.is_err());
///     let err = res.err().unwrap();
///     // Should be network error
///     if let BotejaoError::NetworkError(_) = err{
///
///     }else{
///         panic!(err);
///     }
/// }
/// ```
pub async fn get_menu_from_url(url: &str) -> Result<StructuredWeekMenus, BotejaoError>{
    use crate::unicamp_menu::structs::WeekMenu;
    let mut res = surf::get(url)
        .await
        .map_err(|e| BotejaoError::NetworkError(e.to_string()))?;
    let menu: WeekMenu = res.body_json().await.map_err(|e| BotejaoError::NetworkError(e.to_string()))?;
    let structured = StructuredWeekMenus::from_week_menu(menu)?;
    Ok(structured)
}
