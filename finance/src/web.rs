//use crate::app::model::Point;
use crate::app::utils::{detect_intersections, fetch_sma_async, filter};
use plotters::prelude::*;
use plotters::style::{RED, WHITE};
use rocket::fs::NamedFile;
use rocket::http::Status;
use rocket::{get,routes};
use std::path::Path;

// new
fn generate_chart(
    symbol: &str,
    sma1: Vec<(String, f64)>,
    sma2: Vec<(String, f64)>,
    file_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Create a bitmap backend for the file
    let root = BitMapBackend::new(file_name, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    // Parse dates into indices for X-axis mapping
    let dates: Vec<_> = sma1.iter().map(|(date, _)| date.clone()).collect();
    let min_value = sma1
        .iter()
        .chain(sma2.iter())
        .map(|(_, value)| *value)
        .fold(f64::INFINITY, f64::min);
    let max_value = sma1
        .iter()
        .chain(sma2.iter())
        .map(|(_, value)| *value)
        .fold(f64::NEG_INFINITY, f64::max);

    // Add padding for better visualization
    let padding = (max_value - min_value) * 0.1;
    let adjusted_min = min_value - padding;
    let adjusted_max = max_value + padding;

    let mut chart = ChartBuilder::on(&root)
        .caption(
            format!("Analytics for {}", symbol),
            ("sans-serif", 20).into_font(),
        )
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0..(dates.len() - 1), adjusted_min..adjusted_max)?;

    // Configure mesh with date labels on the X-axis
    chart
        .configure_mesh()
        .x_labels(10)
        .y_labels(10)
        .x_desc("Date")
        .y_desc("Price ($)")
        .x_label_formatter(&|x| {
            if let Some(date) = dates.get(*x) {
                date.clone()
            } else {
                "".to_string()
            }
        })
        .draw()?;

    // Draw SMA1
    chart
        .draw_series(LineSeries::new(
            sma1.iter().enumerate().map(|(i, (_, value))| (i, *value)),
            &BLUE,
        ))?
        .label("SMA1")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

    // Draw SMA2
    chart
        .draw_series(LineSeries::new(
            sma2.iter().enumerate().map(|(i, (_, value))| (i, *value)),
            &RED,
        ))?
        .label("SMA2")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    // Detect intersections
    let (golden, death) = detect_intersections(&sma1, &sma2);

    // Draw Golden Cross points
    chart
        .draw_series(golden.iter().map(|(date, y)| {
            let index = dates.iter().position(|d| d == date).unwrap(); // Find the index of the date
            Circle::new((index, *y), 2, ShapeStyle::from(&GREEN).filled())
        }))?
        .label("Golden Cross")
        .legend(|(x, y)| Circle::new((x + 10, y), 5, ShapeStyle::from(&GREEN).filled()));

    // Draw Death Cross points
    chart
        .draw_series(death.iter().map(|(date, y)| {
            let index = dates.iter().position(|d| d == date).unwrap(); // Find the index of the date
            Circle::new((index, *y), 2, ShapeStyle::from(&YELLOW).filled())
        }))?
        .label("Death Cross")
        .legend(|(x, y)| Circle::new((x + 10, y), 5, ShapeStyle::from(&YELLOW).filled()));

    chart
        .configure_series_labels()
        .border_style(&BLACK)
        .position(SeriesLabelPosition::UpperRight)
        .draw()?;
    
    let golden_text: Vec<String> = golden
        .iter()
        .map(|(date, y)| {
            let date_parts: Vec<&str> = date.split('-').collect();
            let date_without_year = format!("golden:{}-{}", date_parts[1], date_parts[2]); // Extract month and day
            format!("{}: {:.2}", date_without_year, y)
        })
        .collect();

    let death_text: Vec<String> = death
        .iter()
        .map(|(date, y)| {
            let date_parts: Vec<&str> = date.split('-').collect();
            let date_without_year = format!("death:{}-{}", date_parts[1], date_parts[2]); // Extract month and day
            format!("{}: {:.2}", date_without_year, y)
        })
        .collect();

    let text_x_golden = dates.len() as i32 + 10; // X-coordinate for the Golden Cross text box
    let text_x_death = dates.len() as i32 + 150; // X-coordinate for the Death Cross text box
    let text_y_start = max_value; // Starting Y-coordinate for the text box

    for (i, text) in golden_text.iter().enumerate() {
        root.draw(&Text::new(
            text.as_str(), // Dereference the &String to get a &str
            (text_x_golden, text_y_start as i32 - 10 - (i as i32 * 20)), // Adjust position as needed
            ("sans-serif", 12).into_font(),
        ))?;
    }

    for (i, text) in death_text.iter().enumerate() {
        root.draw(&Text::new(
            text.as_str(), // Dereference the &String to get a &str
            (text_x_death, text_y_start as i32 - 10 - (i as i32 * 20)), // Adjust position as needed
            ("sans-serif", 12).into_font(),
        ))?;
    }
    Ok(())
}

// Define the endpoint
#[get("/analytics?<symbol>&<period1>&<period2>")]
async fn analytics(symbol: String, period1: String, period2: String) -> Result<NamedFile, Status> {
    // Fetch SMA data for the first period
    let sma1 = fetch_sma_async(&symbol, &period1).await.map_err(|err| {
        eprintln!("Error fetching SMA1 data for symbol {}: {}", symbol, err);
        Status::InternalServerError
    })?;
    let sma1 = filter(sma1);

    // Fetch SMA data for the second period
    let sma2 = fetch_sma_async(&symbol, &period2).await.map_err(|err| {
        eprintln!("Error fetching SMA2 data for symbol {}: {}", symbol, err);
        Status::InternalServerError
    })?;
    let sma2 = filter(sma2);

    // Generate chart and save to a temporary file
    let file_name = format!("/tmp/{}_analytics_chart.png", symbol);
    if let Err(err) = generate_chart(&symbol, sma1, sma2, &file_name) {
        eprintln!("Error generating chart for symbol {}: {}", symbol, err);
        return Err(Status::InternalServerError);
    }

    // Serve the file
    NamedFile::open(Path::new(&file_name)).await.map_err(|err| {
        eprintln!("Error serving chart file {}: {}", file_name, err);
        Status::InternalServerError
    })
}
pub fn rocket() -> rocket::Rocket<rocket::Build> {
    rocket::build().mount("/", routes![analytics])
}
