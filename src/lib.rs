mod unicamp_menu;
pub use self::unicamp_menu::structs::{StructuredWeekMenus, StructuredDayMenu, Cardapio, MealKind};
pub use self::unicamp_menu::BotejaoError;
pub use self::unicamp_menu::{get_menu, get_menu_from_url};

