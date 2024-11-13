use ratatui::widgets::ListState;
use serde::{Deserialize, Serialize};
pub enum Screen {
    Stock,
    Search,
}
pub enum InputMode {
    Normal,
    Editing,
}
pub struct App {
    pub should_quit: bool,
    pub stock_list: StockList,
    pub search_list: SearchList,
    pub screen: Screen,
    pub input_mode: InputMode,
    pub input: String,
    pub character_index: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Quote {
    pub symbol: String,
    pub name: String,
    pub price: f32,
    #[serde(rename = "changesPercentage")]
    pub changepct: f32,
    pub open: f32,
    #[serde(rename = "dayLow")]
    pub low: f32,
    #[serde(rename = "dayHigh")]
    pub high: f32,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SearchQuote {
    pub symbol: String,
    pub name: String,
    pub currency: String,
}

pub struct StockList {
    pub stocks: Vec<Quote>,
    pub state: ListState,
}

pub struct SearchList {
    pub stocks: Vec<SearchQuote>,
    pub state: ListState,
}
