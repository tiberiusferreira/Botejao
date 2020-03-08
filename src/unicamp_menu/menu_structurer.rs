use crate::unicamp_menu::structs::{StructuredWeekMenus, WeekMenu, Cardapio, MealKind, StructuredDayMenu};
use crate::unicamp_menu::BotejaoError;
use chrono::{NaiveDate};

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_menu_ordering() {
        let cardapio0 = Cardapio{
            data: "2020-03-09".to_string(),
            tipo: "Almoço".to_string(),
            acompanhamento: "".to_string(),
            prato_principal: "".to_string(),
            guarnicao: "".to_string(),
            pts: "".to_string(),
            salada: "".to_string(),
            sobremesa: "".to_string(),
            suco: "".to_string(),
            obs: "".to_string()
        };
        let cardapio1 = Cardapio{
            data: "2020-03-10".to_string(),
            tipo: "Jantar".to_string(),
            acompanhamento: "".to_string(),
            prato_principal: "".to_string(),
            guarnicao: "".to_string(),
            pts: "".to_string(),
            salada: "".to_string(),
            sobremesa: "".to_string(),
            suco: "".to_string(),
            obs: "".to_string()
        };
        let cardapio2 = Cardapio{
            data: "2020-03-11".to_string(),
            tipo: "Jantar".to_string(),
            acompanhamento: "".to_string(),
            prato_principal: "".to_string(),
            guarnicao: "".to_string(),
            pts: "".to_string(),
            salada: "".to_string(),
            sobremesa: "".to_string(),
            suco: "".to_string(),
            obs: "".to_string()
        };
        let week_menu_ordered = WeekMenu{
            menus: vec![cardapio0.clone(), cardapio1.clone(), cardapio2.clone()]
        };
        let week_menu_unordered = WeekMenu{
            menus: vec![cardapio0.clone(), cardapio1.clone(), cardapio2.clone()]
        };
        let structured_ord = StructuredWeekMenus::from_week_menu(week_menu_ordered).unwrap();
        let structured_unord = StructuredWeekMenus::from_week_menu(week_menu_unordered).unwrap();
        assert_eq!(structured_ord.next_menus, structured_unord.next_menus);
        assert!(structured_ord.next_menus[0].day < structured_ord.next_menus[1].day);
        assert!(structured_ord.next_menus[1].day < structured_ord.next_menus[2].day);
    }

    #[test]
    fn test_can_translate_menu_to_struct(){
        let cardapio0 = Cardapio{
            data: "2020-03-09".to_string(),
            tipo: "Almoço".to_string(),
            acompanhamento: "Acomp".to_string(),
            prato_principal: "Prato".to_string(),
            guarnicao: "Guarn".to_string(),
            pts: "Pts".to_string(),
            salada: "Salad".to_string(),
            sobremesa: "Sobrem".to_string(),
            suco: "Suc".to_string(),
            obs: "Some".to_string()
        };
        let week_menu = WeekMenu{
            menus: vec![cardapio0.clone()]
        };
        let structured_week_menu = StructuredWeekMenus::from_week_menu(week_menu).unwrap();
        assert_eq!(structured_week_menu.next_menus.len(), 1);
        let menu = &structured_week_menu.next_menus[0];
        assert_eq!(menu.lunch, Some(cardapio0));
        assert_eq!(menu.dinner, None);
        assert_eq!(menu.veg_lunch, None);
        assert_eq!(menu.veg_dinner, None);
    }

}


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