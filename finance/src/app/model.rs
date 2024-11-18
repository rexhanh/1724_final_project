use ratatui::widgets::ListState;
use serde::{Deserialize, Serialize};
pub enum Screen {
    Stock,
    Search,
    Analytics,
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
    pub status_message: String,
    pub top_list: Vec<Top>,
    pub scroll_offset: usize,
    pub company: Option<Company>,
    pub sma_5days: Vec<ChartDP>,
    pub sma_30days: Vec<ChartDP>,
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
#[derive(Clone)]
pub struct StockList {
    pub stocks: Vec<Quote>,
    pub state: ListState,
}

pub struct SearchList {
    pub stocks: Vec<SearchQuote>,
    pub state: ListState,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChartDP {
    pub date: String,
    #[serde(rename = "sma")]
    pub value: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Company {
    pub symbol: String,
    pub price: f64,
    pub beta: f64,
    #[serde(rename = "volAvg")]
    pub vol_avg: u64,
    #[serde(rename = "mktCap")]
    pub market_cap: u64,
    #[serde(rename = "lastDiv")]
    pub last_dividend: f64,
    pub range: String,
    pub changes: f64,
    #[serde(rename = "companyName")]
    pub company_name: String,
    pub currency: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Top {
    pub symbol: String,
    pub price: f64,
    #[serde(rename = "changesPercentage")]
    pub changespct: f64,
}