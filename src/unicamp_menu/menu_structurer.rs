use crate::unicamp_menu::structs::{StructuredWeekMenus, WeekMenu, Cardapio, MealKind, StructuredDayMenu};
use crate::unicamp_menu::BotejaoError;
use chrono::{NaiveDate};

impl StructuredWeekMenus {
    pub fn from_week_menu(week_menu: WeekMenu) -> Result<Self, BotejaoError> {
        let mut structured_day_menus: Vec<StructuredDayMenu> = vec![];
        // Find a matching day for this menu and insert it there. If none exist, create one.
        'menu_for: for new_menu in week_menu.menus {
            for structured_menu_day in &mut structured_day_menus {
                if structured_menu_day.is_same_day_as(&new_menu)? {
                    structured_menu_day.insert_menu(&new_menu)?;
                    continue 'menu_for;
                }
            }
            let new_day = StructuredDayMenu::new(&new_menu)?;
            structured_day_menus.push(new_day);
        }
        structured_day_menus.sort_by(|first, second| first.day.cmp(&second.day));
        Ok(Self {
            next_menus: structured_day_menus,
        })
    }
}



impl StructuredDayMenu {
    pub fn new(menu: &Cardapio) -> Result<Self, BotejaoError> {
        let date = menu_date(menu)?;
        let mut new = Self {
            day: date,
            lunch: None,
            dinner: None,
            veg_lunch: None,
            veg_dinner: None,
        };
        new.insert_menu(menu)?;
        Ok(new)
    }

    pub fn insert_menu(&mut self, menu: &Cardapio) -> Result<(), BotejaoError> {
        let kind_of_meal = kind_of_meal(&menu)?;
        // If we get duplicated here we don't care, just overwrite it
        match kind_of_meal {
            MealKind::Lunch => self.lunch = Some(menu.clone()),
            MealKind::Dinner => self.dinner = Some(menu.clone()),
            MealKind::VegLunch => self.veg_lunch = Some(menu.clone()),
            MealKind::VegDinner => self.veg_dinner = Some(menu.clone()),
        }
        Ok(())
    }

    pub fn is_same_day_as(&self, menu: &Cardapio) -> Result<bool, BotejaoError> {
        let other_date = menu_date(menu)?;
        Ok(self.day == other_date)
    }
}

pub fn menu_date(menu: &Cardapio) -> Result<NaiveDate, BotejaoError> {
    NaiveDate::parse_from_str(&menu.data, "%Y-%m-%d").map_err(|e| BotejaoError::InvalidDate {
        invalid_date: menu.data.clone(),
        parse_error: e.to_string(),
    })
}

pub fn kind_of_meal(menu: &Cardapio) -> Result<MealKind, BotejaoError> {
    match menu.tipo.as_str() {
        "Almoço" => Ok(MealKind::Lunch),
        "Jantar" => Ok(MealKind::Dinner),
        "Almoço Vegetariano" => Ok(MealKind::VegLunch),
        "Jantar Vegetariano" => Ok(MealKind::VegDinner),
        kind => Err(BotejaoError::InvalidMealKind(kind.to_string())),
    }
}



// #[cfg(test)]
// mod tests {
//     use super::*;
//     use chrono::Weekday;
//
//     #[test]
//     fn can_parse_day_of_week() {
//         let mut empty_menu = Cardapio::default();
//         empty_menu.data = "2020-03-06".to_string();
//         assert_eq!(menu_day_of_week(&empty_menu).unwrap(), Weekday::Fri);
//         empty_menu.data = "w2020-03-06".to_string();
//         assert!(menu_day_of_week(&empty_menu).is_err());
//     }
// }