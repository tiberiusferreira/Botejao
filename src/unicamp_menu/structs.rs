use chrono::NaiveDate;
use serde::{Serialize, Deserialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeekMenu {
    #[serde(rename = "CARDAPIO")]
    pub menus: Vec<Cardapio>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cardapio {
    #[serde(rename = "DATA")]
    pub data: String,
    #[serde(rename = "TIPO")]
    pub tipo: String,
    #[serde(rename = "ACOMPANHAMENTO")]
    pub acompanhamento: String,
    #[serde(rename = "PRATO PRINCIPAL")]
    pub prato_principal: String,
    #[serde(rename = "GUARNICAO")]
    pub guarnicao: String,
    #[serde(rename = "PTS")]
    pub pts: String,
    #[serde(rename = "SALADA")]
    pub salada: String,
    #[serde(rename = "SOBREMESA")]
    pub sobremesa: String,
    #[serde(rename = "SUCO")]
    pub suco: String,
    #[serde(rename = "OBS")]
    pub obs: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructuredDayMenu {
    pub day: NaiveDate,
    pub lunch: Option<Cardapio>,
    pub dinner: Option<Cardapio>,
    pub veg_lunch: Option<Cardapio>,
    pub veg_dinner: Option<Cardapio>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MealKind {
    Lunch,
    Dinner,
    VegLunch,
    VegDinner,
}

#[derive(Debug, Default, PartialEq)]
pub struct StructuredWeekMenus {
    /// Should be ordered by earliest to latest day
    pub next_menus: Vec<StructuredDayMenu>,
}