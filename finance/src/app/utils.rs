use super::model::{Quote, SearchQuote, StockList};
use std::env;

pub fn fetch_search_result(stock: &str) -> Result<Vec<SearchQuote>, reqwest::Error> {
    let api_key = env::var("STOCK_API_KEY").unwrap();
    let body = reqwest::blocking::Client::new()
        .get("https://financialmodelingprep.com/api/v3/search")
        .query(&[("query", stock), ("limit", "10"), ("apikey", &api_key)])
        .send()?
        .json::<Vec<SearchQuote>>()?;
    Ok(body)
}

pub fn fetch_stock(stock: &str) -> Result<Quote, reqwest::Error> {
    let api_key = env::var("STOCK_API_KEY").unwrap();
    let url = String::from("https://financialmodelingprep.com/api/v3/quote/") + stock;
    let body = reqwest::blocking::Client::new()
        .get(url)
        .query(&[("apikey", &api_key)])
        .send()?
        .json::<Vec<Quote>>()?;
    Ok(body[0].clone())
}

pub fn read_saved_quotes_name() -> Result<Vec<String>, std::io::Error> {
    let file = std::fs::File::open("saved.json")?;
    let reader = std::io::BufReader::new(file);
    let names: Vec<String> = serde_json::from_reader(reader)?;
    Ok(names)
}

pub fn save_quotes_name(stocklist: StockList) {
    let names: Vec<String> = stocklist
        .stocks
        .iter()
        .map(|quote| quote.symbol.clone())
        .collect();
    let file = std::fs::File::create("saved.json").unwrap();
    serde_json::to_writer(file, &names).unwrap();
}
