use super::model::{
    ChartDP, Company, HistoricalPrice, NewsData, Quote, SearchQuote, StockData, StockList, Top, Point,
};
use chrono::{
    Datelike, NaiveDateTime, NaiveDate, Utc
};
use scraper::{Html, Selector};
use std::env;
const API_KEY: &str = "uilFVDFWvPNNFgPHkN47tl1vGeusng0H";

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

pub async fn fetch_sma_async(stock: &str, period: &str) -> Result<Vec<ChartDP>, reqwest::Error> {
    let api_key = get_api_key();
    let url = format!("https://financialmodelingprep.com/api/v3/technical_indicator/1day/{}", stock);

    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .query(&[
            ("apikey", &api_key),
            ("period", &period.to_owned()),
            ("type", &"sma".to_owned()),
        ])
        .send()
        .await?;

    let data = response.json::<Vec<ChartDP>>().await?;
    Ok(data)
}

// for tui plot
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

// for web plot: filter and reversse
pub fn filter(data: Vec<ChartDP>) -> Vec<(String, f64)> {
    let current_year = Utc::now().year();

    let mut filtered_data: Vec<_> = data
        .into_iter()
        .filter_map(|item| {
            // Parse the date and remove the time
            if let Ok(parsed_date) = NaiveDate::parse_from_str(&item.date[..10], "%Y-%m-%d") {
                // Only include data for the current year
                if parsed_date.year() == current_year {
                    return Some((parsed_date.format("%Y-%m-%d").to_string(), item.value));
                }
            }
            None
        })
        .collect();

    // Reverse the order to go from least to most recent
    filtered_data.reverse();

    filtered_data
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

pub fn fetch_intraday_data(symbol: &str, current_date:&str) -> Result<Vec<HistoricalPrice>, reqwest::Error> {
    let url = format!(
        "https://financialmodelingprep.com/api/v3/historical-chart/5min/{}",
        symbol
    );
    let response = reqwest::blocking::Client::new()
        .get(&url)
        .query(&[
            ("apikey", API_KEY),
            ("from", &current_date),
            ("to", &current_date),
        ])
        .send()?
        .json::<Vec<HistoricalPrice>>()?;
    Ok(response)
}

pub fn fetch_full_historical_data(symbol: &str) -> Result<StockData, reqwest::Error> {
    let api_key = get_api_key();
    let url = format!(
        "https://financialmodelingprep.com/api/v3/historical-price-full/{}",
        symbol
    );
    let body = reqwest::blocking::Client::new()
        .get(&url)
        .query(&[("apikey", &api_key)])
        .send()?
        .json::<StockData>()?;
    Ok(body)
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

// detect golden cross and death cross
pub fn detect_intersections(
    sma1: &Vec<(String, f64)>,
    sma2: &Vec<(String, f64)>,
) -> (Vec<(String, f64)>, Vec<(String, f64)>) {
    let mut golden_crosses = Vec::new();
    let mut death_crosses = Vec::new();
    let dates: Vec<_> = sma1.iter().map(|(date, _)| date.clone()).collect();

    for i in 0..sma1.len() - 1 {
        let p1 = Point { x: i as f64, y: sma1[i].1 };
        let q1 = Point { x: (i + 1) as f64, y: sma1[i + 1].1 };
        let p2 = Point { x: i as f64, y: sma2[i].1 };
        let q2 = Point { x: (i + 1) as f64, y: sma2[i + 1].1 };

        let denom = (q1.y - p1.y) - (q2.y - p2.y);
        if denom.abs() > 1e-9 {
            let t = (p2.y - p1.y) / denom;
            if t >= 0.0 && t <= 1.0 {
                let x_intersection = p1.x + t * (q1.x - p1.x);
                let y_intersection = p1.y + t * (q1.y - p1.y);

                // Determine the type of crossover
                if sma1[i].1 < sma2[i].1 && sma1[i + 1].1 > sma2[i + 1].1 {
                    // Golden Cross
                    let index = x_intersection.floor() as usize;
                    let date = if index + 1 < dates.len() {
                        dates[index].clone()
                    } else {
                        dates[index].clone()
                    };
                    golden_crosses.push((date, y_intersection));
                } else if sma1[i].1 > sma2[i].1 && sma1[i + 1].1 < sma2[i + 1].1 {
                    // Death Cross
                    let index = x_intersection.floor() as usize;
                    let date = if index + 1 < dates.len() {
                        dates[index].clone()
                    } else {
                        dates[index].clone()
                    };
                    death_crosses.push((date, y_intersection));
                }
            }
        }
    }

    (golden_crosses, death_crosses)
}