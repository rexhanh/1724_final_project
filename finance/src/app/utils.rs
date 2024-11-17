use super::model::{Quote, SearchQuote, ChartDP, Company, Top};
use chrono::{Datelike, NaiveDateTime};
//const API_KEY: &str = "uilFVDFWvPNNFgPHkN47tl1vGeusng0H";
// Bt08M78UNw8jLzvmLk1Bl6s07Gc2rSt6 
// 77iRkUzOmkrSxwfuO3Mb8t7gLd6dP9yg
// H7iSor1eE79j32YkLqY0czsSfJXhUcDr
// Qxk93ZPLycDgwKFc0NILS8yzwTsi8a0y
const API_KEY: &str = "H7iSor1eE79j32YkLqY0czsSfJXhUcDr";
pub fn fetch_search_result(stock: &str) -> Result<Vec<SearchQuote>, reqwest::Error> {
    let body = reqwest::blocking::Client::new()
        .get("https://financialmodelingprep.com/api/v3/search")
        .query(&[("query", stock), ("limit", "10"), ("apikey", API_KEY)])
        .send()?
        .json::<Vec<SearchQuote>>()?;
    Ok(body)
}

pub fn fetch_stock(stock: &str) -> Result<Quote, reqwest::Error> {
    let url = String::from("https://financialmodelingprep.com/api/v3/quote/") + stock;
    let body = reqwest::blocking::Client::new()
        .get(url)
        .query(&[("apikey", API_KEY)])
        .send()?
        .json::<Vec<Quote>>()?;
    Ok(body[0].clone())
}

pub fn fetch_sma(stock: &str, period:&str) -> Result<Vec<ChartDP>, reqwest::Error> {
    let url = String::from("https://financialmodelingprep.com/api/v3/technical_indicator/1day/") + stock;
    let body = reqwest::blocking::Client::new()
        .get(url)
        .query(&[
            ("apikey", API_KEY),
            ("period", period),
            ("type", "sma")
            ])
        .send()?
        .json::<Vec<ChartDP>>()?;
    Ok(body)
}

pub fn parse_chart_point(point: &ChartDP, year: i32) -> Option<(f64, f64)> {
    // Parse the date string to NaiveDateTime
    let datetime = NaiveDateTime::parse_from_str(&point.date, "%Y-%m-%d %H:%M:%S").ok()?;
    
    // Check if the year matches
    if datetime.year() == year {
        // Combine month and day into an f64 in MMDD format
        let month_day = (datetime.month() * 100 + datetime.day()) as f64;
        // Return a tuple with transformed date and value
        Some((month_day, point.value))
    } else {
        None // Return None if the year does not match
    }
}

// get 1 ds<(f64, f64)>, find x-bound, y-bound
// e.g. [(101.0, 100.21), (102.0, 101.21), (103.0, 102.21)] -> ((101.0, 103.0), (100.21, 102.21))
pub fn get_bounds(data1: &[(f64, f64)], data2: &[(f64, f64)]) -> ((f64, f64), (f64, f64)) {
    let mut x_min = f64::MAX;
    let mut x_max = f64::MIN;
    let mut y_min = f64::MAX;
    let mut y_max = f64::MIN;

    // Process the first dataset
    for &(x, y) in data1.iter() {
        if x < x_min { x_min = x; }
        if x > x_max { x_max = x; }
        if y < y_min { y_min = y; }
        if y > y_max { y_max = y; }
    }

    // Process the second dataset
    for &(x, y) in data2.iter() {
        if x < x_min { x_min = x; }
        if x > x_max { x_max = x; }
        if y < y_min { y_min = y; }
        if y > y_max { y_max = y; }
    }

    ((x_min, x_max), (y_min, y_max))
}

pub fn get_company(stock: &str) -> Result<Company, reqwest::Error> {
    let url = String::from("https://financialmodelingprep.com/api/v3/profile/") + stock;
    let body = reqwest::blocking::Client::new()
        .get(url)
        .query(&[("apikey", API_KEY)])
        .send()?
        .json::<Vec<Company>>()?;
    Ok(body[0].clone())
}
// https://financialmodelingprep.com/api/v3/stock_market/gainers
pub fn get_top_gainers() -> Result<Vec<Top>, reqwest::Error> {
    let url = "https://financialmodelingprep.com/api/v3/stock_market/gainers";
    let body = reqwest::blocking::Client::new()
        .get(url)
        .query(&[("apikey", API_KEY)])
        .send()?
        .json::<Vec<Top>>()?;
    Ok(body)
}