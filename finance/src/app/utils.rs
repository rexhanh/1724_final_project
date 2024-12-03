use super::model::{
    ChartDP, Company, HistoricalPrice, NewsData, Quote, SearchQuote, StockData, StockList, Top,
};
use chrono::{
    DateTime, Datelike, Duration, FixedOffset, NaiveDate, NaiveDateTime, NaiveTime, Utc, Weekday,
};
use scraper::{Html, Selector};
use std::env;
const API_KEY: &str = "uilFVDFWvPNNFgPHkN47tl1vGeusng0H";
// BKo3pwdgStNm3rfLEHuit71sK0mvJBCZ
// uilFVDFWvPNNFgPHkN47tl1vGeusng0H
// Bt08M78UNw8jLzvmLk1Bl6s07Gc2rSt6
// 77iRkUzOmkrSxwfuO3Mb8t7gLd6dP9yg
// H7iSor1eE79j32YkLqY0czsSfJXhUcDr
// Qxk93ZPLycDgwKFc0NILS8yzwTsi8a0y

// If the environment variable is set, use it. Otherwise, use the default API key.
fn get_api_key() -> String {
    match env::var("STOCK_API_KEY") {
        Ok(key) => key,
        Err(_) => String::from(API_KEY),
    }
}

pub fn fetch_search_result(stock: &str) -> Result<Vec<SearchQuote>, reqwest::Error> {
    let api_key = get_api_key();
    let body = reqwest::blocking::Client::new()
        .get("https://financialmodelingprep.com/api/v3/search")
        .query(&[("query", stock), ("limit", "10"), ("apikey", &api_key)])
        .send()?
        .json::<Vec<SearchQuote>>()?;
    Ok(body)
}

pub fn fetch_stock(stock: &str) -> Result<Quote, reqwest::Error> {
    let api_key = get_api_key();
    let url = String::from("https://financialmodelingprep.com/api/v3/quote/") + stock;
    let body = reqwest::blocking::Client::new()
        .get(url)
        .query(&[("apikey", api_key)])
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

pub fn fetch_sma(stock: &str, period: &str) -> Result<Vec<ChartDP>, reqwest::Error> {
    let api_key = get_api_key();
    let url =
        String::from("https://financialmodelingprep.com/api/v3/technical_indicator/1day/") + stock;
    let body = reqwest::blocking::Client::new()
        .get(url)
        .query(&[
            ("apikey", &api_key),
            ("period", &period.to_owned()),
            ("type", &"sma".to_owned()),
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
        if x < x_min {
            x_min = x;
        }
        if x > x_max {
            x_max = x;
        }
        if y < y_min {
            y_min = y;
        }
        if y > y_max {
            y_max = y;
        }
    }

    // Process the second dataset
    for &(x, y) in data2.iter() {
        if x < x_min {
            x_min = x;
        }
        if x > x_max {
            x_max = x;
        }
        if y < y_min {
            y_min = y;
        }
        if y > y_max {
            y_max = y;
        }
    }

    ((x_min, x_max), (y_min, y_max))
}

pub fn get_company(stock: &str) -> Result<Company, reqwest::Error> {
    let api_key = get_api_key();
    let url = String::from("https://financialmodelingprep.com/api/v3/profile/") + stock;
    let body = reqwest::blocking::Client::new()
        .get(url)
        .query(&[("apikey", api_key)])
        .send()?
        .json::<Vec<Company>>()?;
    Ok(body[0].clone())
}
// https://financialmodelingprep.com/api/v3/stock_market/gainers
pub fn get_top_gainers() -> Result<Vec<Top>, reqwest::Error> {
    let api_key = get_api_key();
    let url = "https://financialmodelingprep.com/api/v3/stock_market/gainers";
    let body = reqwest::blocking::Client::new()
        .get(url)
        .query(&[("apikey", api_key)])
        .send()?
        .json::<Vec<Top>>()?;
    Ok(body)
}

pub fn fetch_intraday_data(symbol: &str) -> Result<Vec<HistoricalPrice>, reqwest::Error> {
    let current_date = get_most_recent_trading_day_from(Utc::now());
    let current_date_str = current_date.format("%Y-%m-%d").to_string();

    // Fetch intraday data for the current date
    let url = format!(
        "https://financialmodelingprep.com/api/v3/historical-chart/5min/{}",
        symbol
    );
    let response = reqwest::blocking::Client::new()
        .get(&url)
        .query(&[
            ("apikey", API_KEY),
            ("from", &current_date_str),
            ("to", &current_date_str),
        ])
        .send()?
        .json::<Vec<HistoricalPrice>>()?;
    Ok(response)
}

pub fn fetch_historical_data(
    symbol: &str,
    from: &str,
    to: &str,
) -> Result<StockData, reqwest::Error> {
    let api_key = get_api_key();
    let url = format!(
        "https://financialmodelingprep.com/api/v3/historical-price-full/{}",
        symbol
    );
    let body = reqwest::blocking::Client::new()
        .get(&url)
        .query(&[("from", from), ("to", to), ("apikey", &api_key)])
        .send()?
        .json::<StockData>()?;
    Ok(body)
}

pub fn get_month_data_range() -> (String, String) {
    let today = Utc::now().naive_utc().date();
    let one_month_ago = today - Duration::days(30);
    (one_month_ago.to_string(), today.to_string())
}

pub fn get_year_data_range() -> (String, String) {
    let today = Utc::now().naive_utc().date();
    let one_year_ago = today - Duration::days(365);
    (one_year_ago.to_string(), today.to_string())
}

pub fn get_news() -> Result<NewsData, reqwest::Error> {
    let api_key = get_api_key();
    let url = "https://financialmodelingprep.com/api/v3/fmp/articles";
    let body = reqwest::blocking::Client::new()
        .get(url)
        .query(&[("page", "0"), ("size", "10"), ("apikey", &api_key)])
        .send()?
        .json::<NewsData>()?;
    Ok(body)
}

pub fn parse_news(html: Html) -> Vec<String> {
    let selector = Selector::parse("p").expect("Failed to parse selector");
    let mut paragraphs = Vec::new();
    for node in html.select(&selector) {
        paragraphs.push(node.text().collect());
    }
    paragraphs
}

fn get_most_recent_trading_day_from(date: DateTime<Utc>) -> NaiveDate {
    let offset = FixedOffset::west_opt(5 * 60 * 60).unwrap();
    // let offset = FixedOffset::west_opt(0).unwrap();
    let now_with_offset = date.with_timezone(&offset);
    let today_week = now_with_offset.weekday();

    if today_week == Weekday::Sat || today_week == Weekday::Sun {
        return get_last_friday_from(date);
    } else {
        if today_week == Weekday::Mon {
            let midnight = NaiveTime::from_hms_opt(0, 0, 0).unwrap();
            let open_time = NaiveTime::from_hms_opt(9, 30, 0).unwrap();
            if now_with_offset.time() < open_time && now_with_offset.time() >= midnight {
                return get_last_friday_from(date);
            } else {
                return now_with_offset.date_naive();
            }
        } else {
            let midnight = NaiveTime::from_hms_opt(0, 0, 0).unwrap();
            let open_time = NaiveTime::from_hms_opt(9, 30, 0).unwrap();
            if now_with_offset.time() < open_time && now_with_offset.time() >= midnight {
                return now_with_offset.date_naive().pred_opt().unwrap();
            } else {
                return now_with_offset.date_naive();
            }
        }
    }
}

fn get_last_friday_from(date: DateTime<Utc>) -> NaiveDate {
    // Get the current weekday
    let today = date.with_timezone(&FixedOffset::west_opt(5 * 60 * 60).unwrap());

    // Calculate how many days to subtract to get the last Friday
    let days_to_subtract = match today.weekday() {
        Weekday::Mon => 3,
        Weekday::Tue => 4,
        Weekday::Wed => 5,
        Weekday::Thu => 6,
        Weekday::Fri => 7,
        Weekday::Sat => 1,
        Weekday::Sun => 2,
    };

    // Calculate last Friday's date
    (today - Duration::days(days_to_subtract)).date_naive()
}
