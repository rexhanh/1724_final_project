use std::collections::HashMap;

use ratatui::widgets::ListState;
use serde::{Deserialize, Serialize};
pub enum Screen {
    Stock,
    Search,
    Analytics,
    News,
}
pub enum InputMode {
    Normal,
    Editing,
}

pub enum SelectedList {
    None,
    Stock,
    News,
}

pub enum ChartMode {
    Intraday,
    Month,
    Year,
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
    pub news_list: NewsList,
    pub selected_list: SelectedList,
    pub chart_mode: ChartMode,
    pub stock_data_list: HashMap<String, StockHistoricalData>,
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

pub struct NewsList {
    pub news: Vec<News>,
    pub state: ListState,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct News {
    pub title: String,
    pub content: String,
    pub author: String,
    pub date: String,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewsData {
    pub content: Vec<News>,
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HistoricalPrice {
    pub date: String,
    pub open: f64,
    pub low: f64,
    pub high: f64,
    pub close: f64,
    pub volume: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StockData {
    pub symbol: String,
    pub historical: Vec<HistoricalPrice>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StockHistoricalData {
    pub daily: Vec<HistoricalPrice>,
    pub full: Vec<HistoricalPrice>,
}


impl StockHistoricalData {

    pub fn get_thirty_days(&self) -> Vec<HistoricalPrice> {
        self.full.iter().take(30).cloned().collect()
    }

    pub fn get_year_data(&self) -> Vec<HistoricalPrice> {
        self.full.iter().take(365).cloned().collect()
    }
}