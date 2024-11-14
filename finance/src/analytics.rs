use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    text::Line,
    widgets::{Block, Borders, Paragraph, Wrap},
};
use ratatui::prelude::Widget;
use crate::App;
use crate::models::stock::Stock;

/// Render the analytics screen
pub fn render_analytics_screen(
    app: &App,
    selected_stock: &Stock,
    area: Rect,
    buf: &mut Buffer,
) {
    // Define main layout areas for header, main content, and footer
    let [header_area, subheader_area, main_area, footer_area] = Layout::vertical([
        Constraint::Length(2),   // Header height
        Constraint::Length(1),   // Subheader height
        Constraint::Fill(1),     // Main content area takes up remaining space
        Constraint::Length(1),   // Footer height
    ])
    .areas(area);

    // Inside the main content area, divide horizontally into Info, Chart, and Gainers
    let [info_area, chart_area, gainers_area] = Layout::horizontal([
        Constraint::Percentage(30),  // Info area takes 30% of the width
        Constraint::Percentage(40),  // Chart area takes 40% of the width
        Constraint::Percentage(30),  // Gainers area takes 30% of the width
    ])
    .areas(main_area);

    // Render the header, subheader, and footer
    App::render_header(header_area, buf);

    let subheader_text = format!("Analysis for stock {}", selected_stock.name);
    Paragraph::new(Line::raw(subheader_text))
        .alignment(ratatui::layout::Alignment::Center)
        .render(subheader_area, buf);

    // Render the horizontally aligned content areas
    render_stock_info(selected_stock, info_area, buf);
    render_sma_chart(chart_area, buf);  // Includes crossover analysis
    render_top_gainers(gainers_area, buf);

    app.render_footer(footer_area, buf);
}

/// Render stock information table
fn render_stock_info(stock: &Stock, area: Rect, buf: &mut Buffer) {
    let info = format!(
        "Analytics for stock: {}\nIndustry: Tech\nSector: Software\nPrice: ${:.2}\nBeta: 1.2",
        stock.name, stock.price
    );

    let block = Block::new()
        .title(Line::raw("Stock Information").centered())
        .borders(Borders::ALL);

    Paragraph::new(info).block(block).render(area, buf);
}

/// Render a simple SMA line chart with crossover analysis description
fn render_sma_chart(area: Rect, buf: &mut Buffer) {
    // Split the chart area into a description and actual chart section
    let [description_area, actual_chart_area] = Layout::vertical([
        Constraint::Length(3),  // Space for crossover analysis description
        Constraint::Fill(1),    // Remaining space for the actual chart
    ])
    .areas(area);

    // Description of crossover analysis
    let description = "Crossover Analysis: When the 5-day SMA crosses above the 20-day SMA, it signals a potential uptrend. When it crosses below, it may signal a downtrend.";

    let description_block = Block::new()
        .title(Line::raw("Crossover Analysis").centered())
        .borders(Borders::ALL);

    Paragraph::new(description)
        .block(description_block)
        .wrap(Wrap { trim: false })
        .render(description_area, buf);

    // Placeholder for the actual SMA chart
    let chart_block = Block::new()
        .title(Line::raw("SMA Chart").centered())
        .borders(Borders::ALL);

    let chart_info = "5-day SMA: 125.4, 20-day SMA: 123.8"; // Placeholder data
    Paragraph::new(chart_info)
        .block(chart_block)
        .render(actual_chart_area, buf);
}

/// Render top gainers table
fn render_top_gainers(area: Rect, buf: &mut Buffer) {
    let gainers = "AAPL - $130\nMSFT - $250\nGOOGL - $1900"; // Placeholder data

    let block = Block::new()
        .title(Line::raw("Top Gainers").centered())
        .borders(Borders::ALL);

    Paragraph::new(gainers).block(block).render(area, buf);
}