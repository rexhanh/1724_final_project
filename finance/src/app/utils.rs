use super::model::{Quote, SearchQuote};
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
        .query(&[("apikey", api_key)])
        .send()?
        .json::<Vec<Quote>>()?;
    Ok(body[0].clone())
}
