use std::collections::HashMap;

use chrono::{Datelike, NaiveDate, Timelike, NaiveDateTime};
use color_eyre::Result;
use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Layout, Position, Rect},
    style::{Color, Style, Stylize},
    symbols::{self},
    text::{Line, Span},
    widgets::{
        Axis, Block, Borders, Chart, Dataset, GraphType, List, ListItem, ListState, Padding,
        Paragraph, StatefulWidget, Widget, Wrap,
    },
    DefaultTerminal, Frame,
};
mod model;
pub use model::{
    App, ChartMode, InputMode, NewsList, Quote, Screen, SearchList, SelectedList,
    StockHistoricalData, StockList,
};
mod utils;
use scraper::Html;
pub use utils::{
    fetch_historical_data, fetch_intraday_data, fetch_search_result, fetch_sma, fetch_stock,
    get_bounds, get_company, get_month_data_range, get_news, get_top_gainers, get_year_data_range,
    parse_chart_point, parse_news, read_saved_quotes_name, save_quotes_name, 
};
impl StockList {
    fn new() -> Self {
        Self {
            stocks: Vec::new(),
            state: ListState::default(),
        }
    }
    fn add_stock(&mut self, stock: Quote) {
        self.stocks.push(stock);
    }
}

impl SearchList {
    fn new() -> Self {
        Self {
            stocks: Vec::new(),
            state: ListState::default(),
        }
    }

    fn clear(&mut self) {
        self.stocks = vec![];
        self.state = ListState::default();
    }
}

impl NewsList {
    fn new() -> Self {
        match get_news() {
            Ok(news) => {
                let n = news.content;
                Self {
                    news: n,
                    state: ListState::default(),
                }
            }
            Err(_) => Self {
                news: vec![],
                state: ListState::default(),
            },
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    fn new() -> Self {
        let mut stock_list = StockList::new();
        let mut stock_data_list: HashMap<String, StockHistoricalData> = HashMap::new();
        // Read saved quotes from file and add them to the stock list
        match read_saved_quotes_name() {
            Ok(names) => {
                for name in names {
                    let stock = fetch_stock(&name);
                    let (from, to) = get_month_data_range();
                    let monthly_data = fetch_historical_data(&name, &from, &to).expect("Error");
                    let intra_day_data = fetch_intraday_data(&name).expect("Error");
                    let (from, to) = get_year_data_range();
                    let yearly_data = fetch_historical_data(&name, &from, &to).expect("Error");
                    stock_data_list.insert(
                        name.clone(),
                        StockHistoricalData {
                            monthly: monthly_data.historical,
                            daily: intra_day_data,
                            yearly: yearly_data.historical,
                        },
                    );
                    match stock {
                        Ok(stock) => {
                            stock_list.add_stock(stock);
                        }
                        Err(_) => {}
                    }
                }
            }
            Err(_) => {}
        }

        let search_list = SearchList::new();
        // Fetch the top gainers when initializing the app
        let top_lst = match get_top_gainers() {
            Ok(gainers) => gainers,
            Err(_) => vec![], // Handle any errors by setting an empty list
        };

        let news_list = NewsList::new();
        Self {
            should_quit: false,
            stock_list,
            search_list,
            screen: Screen::Stock,
            input_mode: InputMode::Normal,
            input: String::new(),
            character_index: 0,
            status_message: String::new(),
            top_list: top_lst,
            scroll_offset: 0,
            company: None,
            sma_5days: vec![],
            sma_30days: vec![],
            news_list,
            selected_list: SelectedList::Stock,
            chart_mode: ChartMode::Intraday, // Default mode set to Intraday
            stock_data_list,
        }
    }

    fn scroll_down(&mut self) {
        self.scroll_offset = (self.scroll_offset + 1).min(self.top_list.len());
    }

    fn scroll_up(&mut self) {
        self.scroll_offset = self.scroll_offset.saturating_sub(1);
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        while !self.should_quit {
            terminal.draw(|frame| self.draw(frame))?;
            if let Event::Key(key) = event::read()? {
                match self.screen {
                    Screen::Stock => self.handle_stock_screen_key(key),
                    Screen::Search => self.handle_search_screen_key(key),
                    Screen::Analytics => self.handle_analytics_screen_key(key),
                    Screen::News => {
                        self.handle_news_screen_key(key);
                    }
                }
            };
        }
        Ok(())
    }

    fn handle_stock_screen_key(&mut self, key: KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }
        match key.code {
            KeyCode::Esc => self.exit(),
            KeyCode::Down => self.select_next(),
            KeyCode::Up => self.select_previous(),
            KeyCode::Left => self.select_left(), //self.select_none(),
            KeyCode::Right => self.select_right(),
            KeyCode::Char('m') => {
                self.chart_mode = ChartMode::Month;
            }
            KeyCode::Char('d') => {
                self.chart_mode = ChartMode::Intraday;
            }
            KeyCode::Char('y') => {
                self.chart_mode = ChartMode::Year;
            }
            KeyCode::Char('s') => {
                self.search_list.clear();
                self.screen = Screen::Search;
            }
            KeyCode::Enter => {
                self.handle_enter();
            }
            _ => {}
        }
    }
    fn handle_enter(&mut self) {
        match self.selected_list {
            SelectedList::None => {}
            SelectedList::Stock => {
                self.to_analytics_screen();
            }
            SelectedList::News => {
                self.to_news_screen();
            }
        }
    }
    fn to_news_screen(&mut self) {
        self.screen = Screen::News;
    }
    fn to_analytics_screen(&mut self) {
        if self.stock_list.state.selected().is_some() {
            // If a stock is selected, go to the analytics screen
            // get selected stock
            let i = self.stock_list.state.selected().unwrap();
            let selected_stock = &self.stock_list.stocks[i];
            // get data for analytics sreen
            self.company = Some(get_company(&selected_stock.symbol).unwrap());
            self.sma_5days = fetch_sma(&selected_stock.symbol, "5").unwrap();
            self.sma_30days = fetch_sma(&selected_stock.symbol, "30").unwrap();

            // go to analytics screen
            self.screen = Screen::Analytics;
        } else {
            // If no stock is selected, set a warning message
            self.status_message = String::from("Please select a stock before entering analytics.");
        }
    }

    fn handle_search_screen_key(&mut self, key: KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }
        match self.input_mode {
            InputMode::Normal => match key.code {
                KeyCode::Esc => self.exit(),
                KeyCode::Char('s') => {
                    self.screen = Screen::Stock;
                }
                KeyCode::Char('i') => {
                    self.input_mode = InputMode::Editing;
                }
                KeyCode::Char('q') => {
                    self.input_mode = InputMode::Normal;
                }
                KeyCode::Down => self.select_next_search(),
                KeyCode::Up => self.select_previous_search(),
                KeyCode::Left => self.select_none_search(),
                KeyCode::Enter => self.add_stock(),
                _ => {}
            },
            InputMode::Editing => match key.code {
                KeyCode::Enter => {
                    self.submit_message(self.input.clone());
                }
                KeyCode::Esc => {
                    self.input_mode = InputMode::Normal;
                }
                KeyCode::Char(to_insert) => self.enter_char(to_insert),
                KeyCode::Backspace => self.delete_char(),
                KeyCode::Left => self.move_cursor_left(),
                KeyCode::Right => self.move_cursor_right(),
                _ => {}
            },
        }
    }

    // Handle keys for Analytics screen
    fn handle_analytics_screen_key(&mut self, key: KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => self.exit(),
            KeyCode::Backspace | KeyCode::Char('h') => {
                self.screen = Screen::Stock;
            }
            KeyCode::Down => self.scroll_down(), // Scroll down on Down arrow key
            KeyCode::Up => self.scroll_up(),     // Scroll up on Up arrow key
            _ => {}
        }
    }
    fn handle_news_screen_key(&mut self, key: KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }
        match key.code {
            KeyCode::Esc => self.exit(),
            KeyCode::Char('h') => {
                self.screen = Screen::Stock;
            }
            _ => {}
        }
    }
    fn select_next(&mut self) {
        // self.stock_list.state.select_next();
        match self.selected_list {
            SelectedList::None => {}
            SelectedList::Stock => {
                self.stock_list.state.select_next();
            }
            SelectedList::News => {
                self.news_list.state.select_next();
            }
        }
    }

    fn select_previous(&mut self) {
        match self.selected_list {
            SelectedList::None => {}
            SelectedList::Stock => {
                self.stock_list.state.select_previous();
            }
            SelectedList::News => {
                self.news_list.state.select_previous();
            }
        }
    }

    fn select_left(&mut self) {
        match self.selected_list {
            SelectedList::None => {}
            SelectedList::Stock => {
                self.stock_list.state.select(None);
                self.selected_list = SelectedList::None;
            }
            SelectedList::News => {
                self.news_list.state.select(None);
                self.stock_list.state.select(Some(0));
                self.selected_list = SelectedList::Stock;
            }
        }
    }
    fn select_right(&mut self) {
        match self.selected_list {
            SelectedList::None => {
                self.stock_list.state.select(Some(0));
                self.selected_list = SelectedList::Stock;
            }
            SelectedList::Stock => {
                self.stock_list.state.select(None);
                self.news_list.state.select(Some(0));
                self.selected_list = SelectedList::News;
            }
            SelectedList::News => {}
        }
    }
    fn select_next_search(&mut self) {
        self.search_list.state.select_next();
    }

    fn select_previous_search(&mut self) {
        self.search_list.state.select_previous();
    }
    fn select_none_search(&mut self) {
        self.search_list.state.select(None);
    }

    fn exit(&mut self) {
        self.should_quit = true;
        save_quotes_name(self.stock_list.clone());
    }
    fn add_stock(&mut self) {
        if let Some(i) = self.search_list.state.selected() {
            let stock_symbol = self.search_list.stocks[i].clone().symbol;
            let stock = fetch_stock(&stock_symbol);
            // Fetch Historical data for the stock
            let (from, to) = get_month_data_range();
            let monthly_data = fetch_historical_data(&stock_symbol, &from, &to).expect("Error");
            let intra_day_data = fetch_intraday_data(&stock_symbol).expect("Error");
            let (from, to) = get_year_data_range();
            let yearly_data = fetch_historical_data(&stock_symbol, &from, &to).expect("Error");
            self.stock_data_list.insert(
                stock_symbol,
                StockHistoricalData {
                    monthly: monthly_data.historical,
                    daily: intra_day_data,
                    yearly: yearly_data.historical,
                },
            );

            match stock {
                Ok(stock) => {
                    self.stock_list.add_stock(stock);
                    self.screen = Screen::Stock;
                }
                Err(_) => {}
            }
        }
    }
}

// Implementaion for screen rendering
// Each screen will render components differently
impl App {
    // Main draw function, this is called in the loop
    fn draw(&mut self, frame: &mut Frame) {
        match self.screen {
            Screen::Stock => self.draw_stock_screen(frame),
            Screen::Search => self.draw_search_screen(frame),
            Screen::Analytics => {
                if self.stock_list.state.selected().is_some() {
                    self.draw_analytics_screen(frame);
                }
            }
            Screen::News => {
                self.draw_news_screen(frame);
            }
        }
    }

    fn draw_stock_screen(&mut self, frame: &mut Frame) {
        let [header_area, main_area, _footer_area] = Layout::vertical([
            Constraint::Length(2),
            Constraint::Fill(1),
            Constraint::Length(2),
        ])
        .areas(frame.area());

        let [list_area, item_area] =
            Layout::horizontal([Constraint::Percentage(10), Constraint::Percentage(90)])
                .areas(main_area);

        let [_chart_area, info_main_area] =
            Layout::vertical([Constraint::Percentage(70), Constraint::Percentage(30)])
                .areas(item_area);

        let [info_area, _news_area] =
            Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
                .areas(info_main_area);
        App::render_header(header_area, frame.buffer_mut());
        self.render_list(list_area, frame.buffer_mut());
        self.render_selected_item(info_area, frame.buffer_mut());
        self.render_chart(_chart_area, frame);
        App::render_footer(_footer_area, frame.buffer_mut(), Screen::Stock);
        self.render_news(_news_area, frame);
    }

    fn draw_search_screen(&mut self, frame: &mut Frame) {
        let [input_area, main_area, _footer_area] = Layout::vertical([
            Constraint::Length(5),
            Constraint::Fill(1),
            Constraint::Length(1),
        ])
        .areas(frame.area());
        let block = Block::new()
            .title(Line::raw("Search").centered())
            .borders(Borders::ALL)
            .border_set(symbols::border::THICK);

        Paragraph::new(self.input.clone())
            .block(block)
            .render(input_area, frame.buffer_mut());

        match self.input_mode {
            InputMode::Normal => {}
            InputMode::Editing => {
                frame.set_cursor_position(Position::new(
                    input_area.x + 1 + self.byte_index() as u16,
                    input_area.y + 1,
                ));
            }
        }
        self.render_search_result(main_area, frame.buffer_mut());
        self.render_search_footer(_footer_area, frame.buffer_mut());
    }

    fn draw_analytics_screen(&self, frame: &mut Frame) {
        if let Some(i) = self.stock_list.state.selected() {
            let selected_stock = &self.stock_list.stocks[i];
            // Define main layout areas for header, main content, and footer
            let [header_area, subheader_area, main_area, footer_area] = Layout::vertical([
                Constraint::Length(2), // Header height
                Constraint::Length(1), // Subheader height
                Constraint::Fill(1),   // Main content area takes up remaining space
                Constraint::Length(1), // Footer height
            ])
            .areas(frame.area());

            // Inside the main content area, divide horizontally into Info, Chart, and Gainers
            let [info_area, chart_area, gainers_area] = Layout::horizontal([
                Constraint::Percentage(30), // Info area takes 30% of the width
                Constraint::Percentage(40), // Chart area takes 40% of the width
                Constraint::Percentage(30), // Gainers area takes 30% of the width
            ])
            .areas(main_area);
            App::render_header(header_area, frame.buffer_mut());
            let subheader_text = format!("Analysis for stock {}", selected_stock.name);
            Paragraph::new(Line::raw(subheader_text))
                .alignment(ratatui::layout::Alignment::Center)
                .render(subheader_area, frame.buffer_mut());
            self.render_company_info(info_area, frame);
            self.render_sma_chart(chart_area, frame); // Includes crossover analysis
            self.render_top_gainers(gainers_area, frame);

            // TODO Might need a new render for this screen
            App::render_footer(footer_area, frame.buffer_mut(), Screen::Analytics);
        }
    }

    fn draw_news_screen(&mut self, frame: &mut Frame) {
        let [_header_area, main_area, _footer_area] = Layout::vertical([
            Constraint::Length(2),
            Constraint::Fill(1),
            Constraint::Length(1),
        ])
        .areas(frame.area());
        let selected_news = self.news_list.state.selected().unwrap();
        let news = self.news_list.news[selected_news].clone();
        let block = Block::new()
            .title(Line::raw(news.title).centered())
            .borders(Borders::ALL)
            .border_set(symbols::border::THICK);
        let t = self.news_list.state.selected();
        match t {
            Some(i) => {
                let news = self.news_list.news[i].clone();
                let document = Html::parse_fragment(&news.content);
                let paragraphs = parse_news(document).join("\n\n");
                let res = "Author: ".to_owned()
                    + &news.author
                    + "\nDate: "
                    + &news.date
                    + "\n\n"
                    + &paragraphs;
                Paragraph::new(res)
                    .block(block)
                    .wrap(Wrap { trim: true })
                    .render(main_area, frame.buffer_mut());
            }
            None => {
                Paragraph::new("No news selected")
                    .wrap(Wrap { trim: true })
                    .render(main_area, frame.buffer_mut());
            }
        }
        App::render_header(_header_area, frame.buffer_mut());
        App::render_footer(_footer_area, frame.buffer_mut(), Screen::News);
    }
}

// Implementation of each views' rendering
impl App {
    fn render_header(area: Rect, buf: &mut Buffer) {
        Paragraph::new("Finance").centered().render(area, buf);
    }

    fn render_list(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::new()
            .title(Line::raw("Stocks").left_aligned().bold())
            .borders(Borders::ALL)
            .border_set(symbols::border::THICK);

        let items: Vec<ListItem> = self
            .stock_list
            .stocks
            .iter()
            .map(|stock| ListItem::new(stock.symbol.clone()))
            .collect();

        let list = List::new(items).block(block).highlight_symbol(">");

        StatefulWidget::render(list, area, buf, &mut self.stock_list.state);
    }
    // TODO Need to implement the rendering of the selected item
    // Currently, it just shows the name and price of the selected item
    fn render_selected_item(&self, area: Rect, buf: &mut Buffer) {
        // We get the info depending on the item's state.
        let info = if let Some(i) = self.stock_list.state.selected() {
            let stock = &self.stock_list.stocks[i];
            format!(
                "Name: {}\nPrice: ${}\nOpen: ${}\nChange Percentage: {}%",
                stock.symbol, stock.price, stock.open, stock.changepct
            )
        } else {
            "Nothing selected...".to_string()
        };

        // We show the list item's info under the list in this paragraph
        let block = Block::new()
            .title(Line::raw("Stock Info").centered())
            .borders(Borders::ALL)
            .border_set(symbols::border::THICK)
            .padding(Padding::horizontal(1));

        // We can now render the item info
        Paragraph::new(info)
            .block(block)
            .wrap(Wrap { trim: false })
            .render(area, buf);
    }

    fn render_chart(&self, area: Rect, frame: &mut Frame) {
        if let Some(i) = self.stock_list.state.selected() {
            let symbol = &self.stock_list.stocks[i].symbol;

            // Fetch data based on the chart mode
            let (chart_data, title, x_axis_labels, x_min, x_max) = match self.chart_mode {
                ChartMode::Intraday => {
                    // Fetch intraday data using the updated function
                    let intraday_data = fetch_intraday_data(symbol).unwrap_or_else(|_| vec![]);
                
                    // Get the date of the intraday data
                    let chart_date = &intraday_data.first().unwrap().date[..10]; // Extract YYYY-MM-DD
                
                    // Prepare data points
                    let mut dps: Vec<(f64, f64)> = Vec::new();
                
                    // Define trading hours (static x-axis labels)
                    let trading_hours = vec![
                        "09:30", "10:00", "11:00", "12:00", "13:00", "14:00", "15:00", "16:00",
                    ];
                
                    // Total trading minutes: 390 (from 09:30 to 16:00)
                    let total_minutes = 390;
                
                    for entry in intraday_data.iter() {
                        if let Ok(datetime) =
                            NaiveDateTime::parse_from_str(&entry.date, "%Y-%m-%d %H:%M:%S")
                        {
                            // Calculate minutes since market open (9:30 AM = 570 minutes since midnight)
                            let minutes_since_open = datetime.hour() * 60 + datetime.minute() - 570;
                
                            if minutes_since_open <= total_minutes {
                                // Scale minutes_since_open to the range of x-axis labels
                                let x_index = (minutes_since_open as f64 / total_minutes as f64) * 7.0; // Scale to 7 intervals
                                dps.push((x_index, entry.close));
                            }
                        }
                    }
                
                    // Generate static x-axis labels
                    let x_labels: Vec<Line> = trading_hours
                        .iter()
                        .map(|&label| Line::from(Span::raw(label)))
                        .collect();
                
                    // Set x-axis bounds based on the number of labels
                    let x_min = 0.0;
                    let x_max = 7.0;
                
                    (
                        dps,
                        format!("Intraday Data for {}", chart_date),
                        x_labels,
                        x_min,
                        x_max,
                    )
                }

                    
                ChartMode::Month => {
                    // Fetch 30-day historical data
                    let month_data = self.stock_data_list.get(symbol).unwrap();
                    // Prepare 30-day data points and labels
                    let mut dps: Vec<(f64, f64)> = Vec::new();
                    let mut monday_labels: Vec<Line> = Vec::new();

                    for (index, entry) in month_data.monthly.iter().rev().enumerate() {
                        // Parse the date string to NaiveDate
                        if let Ok(date) = NaiveDate::parse_from_str(&entry.date, "%Y-%m-%d") {
                            // Add the close price to data points in reverse order
                            dps.push((index as f64, entry.close));

                            // Check if the date is a Monday
                            if date.weekday() == chrono::Weekday::Mon {
                                // Format the date as "MMM DD" and add to labels
                                let label = date.format("%b %d").to_string();
                                monday_labels.push(Line::from(Span::raw(label)));
                            }
                        }
                    }

                    // Calculate y-axis bounds
                    let x_min = 0.0;
                    let x_max = dps.len() as f64 - 1.0;

                    (
                        dps,
                        "1-Month Price History".to_string(),
                        monday_labels,
                        x_min,
                        x_max,
                    )
                }

                ChartMode::Year => {
                    // Fetch year data
                    let year_data = self.stock_data_list.get(symbol).unwrap();

                    // Prepare data points and labels
                    let mut dps: Vec<(f64, f64)> = Vec::new();
                    let mut x_labels: Vec<Line> = Vec::new();

                    for (index, entry) in year_data.yearly.iter().rev().enumerate() {
                        if let Ok(date) = NaiveDate::parse_from_str(&entry.date, "%Y-%m-%d") {
                            // Add the close price to data points in reverse order
                            dps.push((index as f64, entry.close));

                            // Add label every three months
                            if index % 60 == 0 {
                                let label = date.format("%b %Y").to_string();
                                x_labels.push(Line::from(Span::raw(label)));
                            }
                        }
                    }

                    // Calculate x-axis bounds
                    let x_min = 0.0;
                    let x_max = dps.len() as f64 - 1.0;

                    (
                        dps,
                        "1-Year Price History".to_string(),
                        x_labels,
                        x_min,
                        x_max,
                    )
                }
            };

            // Calculate y-axis bounds
            let y_min = chart_data
                .iter()
                .map(|(_, y)| *y)
                .fold(f64::INFINITY, f64::min);
            let y_max = chart_data
                .iter()
                .map(|(_, y)| *y)
                .fold(f64::NEG_INFINITY, f64::max);

            // Define the chart
            let chart = Chart::new(vec![Dataset::default()
                .marker(symbols::Marker::Braille)
                .style(Style::default().fg(Color::Yellow))
                .graph_type(GraphType::Line)
                .data(&chart_data)])
            .block(
                Block::default()
                    .title(Line::raw(title).centered())
                    .borders(Borders::ALL),
            )
            .x_axis(
                Axis::default()
                    .title("Time")
                    .style(Style::default().gray())
                    .bounds([x_min, x_max])
                    .labels(x_axis_labels),
            )
            .y_axis(
                Axis::default()
                    .title("Price")
                    .style(Style::default().gray())
                    .bounds([y_min, y_max])
                    .labels(vec![
                        Line::from(format!("{:.2}", y_min)),
                        Line::from(format!("{:.2}", (y_min + y_max) / 2.0)),
                        Line::from(format!("{:.2}", y_max)),
                    ]),
            );

            frame.render_widget(chart, area);
        } else {
            let block = Block::default()
                .title(Line::raw("Chart").centered())
                .borders(Borders::ALL)
                .border_set(symbols::border::THICK);

            let paragraph = Paragraph::new("Nothing selected...").block(block);
            frame.render_widget(paragraph, area);
        }
    }

    fn render_footer(area: Rect, buf: &mut Buffer, screen: Screen) {
        match screen {
            Screen::Stock => {
                Paragraph::new(
                    "↓↑ to move, ← → to switch between stock and news, s to search, d to view daily chart, m to view monthly chart, 
                    y to view yearly chart, Enter to analytics or news, Esc to quit            ",
                )
                .centered()
                .render(area, buf);
            }
            _ => {
                Paragraph::new("h to return to home, Esc to quit")
                    .centered()
                    .render(area, buf);
            }
        }
    }

    fn render_news(&mut self, area: Rect, frame: &mut Frame) {
        let block = Block::new()
            .title(Line::raw("General News").centered())
            .borders(Borders::ALL)
            .border_set(symbols::border::THICK);

        // Define the news content
        let items: Vec<ListItem> = self
            .news_list
            .news
            .iter()
            .map(|news| ListItem::new(news.title.clone()))
            .collect();
        let list = List::new(items).block(block).highlight_symbol(">");
        StatefulWidget::render(list, area, frame.buffer_mut(), &mut self.news_list.state);
    }
    fn render_search_footer(&self, area: Rect, buf: &mut Buffer) {
        match self.input_mode {
            InputMode::Normal => self.render_normal_footer(area, buf),
            InputMode::Editing => self.render_editing_footer(area, buf),
        }
    }

    fn render_normal_footer(&self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("Use q to quit, use s return to stock screen, use i to insert")
            .centered()
            .render(area, buf);
    }

    fn render_editing_footer(&self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("Use Enter to submit, use Esc to quit editting")
            .centered()
            .render(area, buf);
    }

    fn render_search_result(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::new()
            .title(Line::raw("Result").centered())
            .borders(Borders::ALL)
            .border_set(symbols::border::THICK);

        let items: Vec<ListItem> = self
            .search_list
            .stocks
            .iter()
            .map(|stock| ListItem::new(stock.symbol.clone()))
            .collect();
        let list = List::new(items).block(block).highlight_symbol(">");
        StatefulWidget::render(list, area, buf, &mut self.search_list.state);
    }

    fn render_company_info(&self, area: Rect, frame: &mut Frame) {
        // Assuming `get_company` returns a Result with `Company` instance
        // let company = get_company(&selected_quote.symbol).unwrap();
        let company = self.company.as_ref().unwrap(); // TODO get selected stock

        // Display company fields, 1 field per line
        let company_info = format!(
            "Symbol: {}\nCompany_name: {}\nPrice: {}\nbeta: {}\n
            Volumn Avg: {}\nMarket Cap: {}\n
            Last Dividend: {}\nRange: {}\nChanges: {}\nCurrency: {}",
            company.symbol,
            company.company_name,
            company.price,
            company.beta,
            company.vol_avg,
            company.market_cap,
            company.last_dividend,
            company.range,
            company.changes,
            company.currency,
        );

        // Render this information within the given area using Ratatui
        let paragraph = Paragraph::new(company_info)
            .block(Block::default().title("Company Info").borders(Borders::ALL))
            .wrap(Wrap { trim: true });

        frame.render_widget(paragraph, area);
    }

    fn render_sma_chart(&self, area: Rect, frame: &mut Frame) {
        // process the SMA data
        let sma_5days = &self.sma_5days;
        let sma_20days = &self.sma_30days;

        // Filter data to only include this year's entries
        let current_year = 2024;
        let dps: Vec<(f64, f64)> = sma_5days
            .iter()
            .filter_map(|data| parse_chart_point(&data, current_year))
            .collect();

        let dps2: Vec<(f64, f64)> = sma_20days
            .iter()
            .filter_map(|data| parse_chart_point(&data, current_year))
            .collect();

        // Calculate chart bounds
        let ((x_min, x_max), (y_min, y_max)) = get_bounds(&dps, &dps2);

        // Define the chart with datasets
        let chart = Chart::new(vec![
            Dataset::default()
                .name("10-day SMA")
                .marker(symbols::Marker::Braille)
                .style(Style::default().fg(Color::Cyan))
                .graph_type(GraphType::Line)
                .data(&dps),
            Dataset::default()
                .name("20-day SMA")
                .marker(symbols::Marker::Braille)
                .style(Style::default().fg(Color::Yellow))
                .graph_type(GraphType::Line)
                .data(&dps2),
        ])
        .block(Block::bordered().title("Simple Moving Average (SMA) Graph 2024: (X-axis: MMDD)"))
        .x_axis(
            Axis::default()
                .title("X Axis: Time")
                .style(Style::default().gray())
                .bounds([x_min, x_max])
                .labels([
                    Line::from(format!("{:.0}", x_min)),
                    Line::from(format!("{:.0}", (x_min + x_max) / 2.0)),
                    Line::from(format!("{:.0}", x_max)),
                ]),
        )
        .y_axis(
            Axis::default()
                .title("Y Axis: Stock Price")
                .style(Style::default().gray())
                .bounds([y_min, y_max])
                .labels([
                    Line::from(format!("{:.2}", y_min)),
                    Line::from(format!("{:.2}", (y_min + y_max) / 2.0)),
                    Line::from(format!("{:.2}", y_max)),
                ]),
        )
        .hidden_legend_constraints((Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)));
        // Render the chart in the remaining area
        frame.render_widget(chart, area);
    }

    fn render_top_gainers(&self, area: Rect, frame: &mut Frame) {
        // Define how many items to display based on the area height
        let max_visible_items = (area.height as usize).saturating_sub(3); // Adjust for block padding and footer

        // Calculate scrollbar height and position
        let total_items = self.top_list.len();
        let scrollbar_height = max_visible_items.min(area.height as usize - 3);
        let scrollbar_position = if total_items > max_visible_items {
            (self.scroll_offset * (scrollbar_height - 1)) / (total_items - max_visible_items)
        } else {
            0
        };

        // Slice the top gainers list based on the scroll offset
        let visible_gainers = if self.top_list.is_empty() {
            vec!["No gainers available.".to_string()]
        } else {
            self.top_list
                .iter()
                .skip(self.scroll_offset) // Start from the scroll offset
                .take(max_visible_items) // Only take the items that fit in the visible area
                .map(|gainer| {
                    format!(
                        "{} - Price: ${:.2}, Change: {:.2}%",
                        gainer.symbol, gainer.price, gainer.changespct
                    )
                })
                .collect::<Vec<String>>()
        };

        // Join the visible items with line breaks
        let gainers_info = visible_gainers.join("\n");

        // Render the top gainers block
        let block = Block::new()
            .title(Line::raw("Top Gainers (Use ↑↓ to scroll)").centered())
            .borders(Borders::ALL);

        // Render the paragraph with gainers information
        Paragraph::new(gainers_info)
            .block(block)
            .render(area, frame.buffer_mut());

        // Render the scrollbar on the right side of the block
        let scrollbar_content: String = (0..scrollbar_height)
            .map(|i| {
                if i == scrollbar_position {
                    "█\n" // Scroll handle
                } else {
                    "░\n" // Empty scrollbar line
                }
            })
            .collect();

        let scrollbar_paragraph =
            Paragraph::new(scrollbar_content).alignment(ratatui::layout::Alignment::Left);

        let scrollbar_area = Rect {
            x: area.x + area.width - 1,
            y: area.y + 1,
            width: 1,
            height: area.height - 2,
        };

        frame.render_widget(scrollbar_paragraph, scrollbar_area);
    }
}

impl App {
    // Taken from https://ratatui.rs/examples/apps/user_input/
    fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.character_index.saturating_sub(1);
        self.character_index = self.clamp_cursor(cursor_moved_left);
    }

    fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.character_index.saturating_add(1);
        self.character_index = self.clamp_cursor(cursor_moved_right);
    }

    fn enter_char(&mut self, new_char: char) {
        let index = self.byte_index();
        self.input.insert(index, new_char);
        self.move_cursor_right();
    }

    /// Returns the byte index based on the character position.
    ///
    /// Since each character in a string can be contain multiple bytes, it's necessary to calculate
    /// the byte index based on the index of the character.
    fn byte_index(&self) -> usize {
        self.input
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.character_index)
            .unwrap_or(self.input.len())
    }

    fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.character_index != 0;
        if is_not_cursor_leftmost {
            // Method "remove" is not used on the saved text for deleting the selected char.
            // Reason: Using remove on String works on bytes instead of the chars.
            // Using remove would require special care because of char boundaries.

            let current_index = self.character_index;
            let from_left_to_current_index = current_index - 1;

            // Getting all characters before the selected character.
            let before_char_to_delete = self.input.chars().take(from_left_to_current_index);
            // Getting all characters after selected character.
            let after_char_to_delete = self.input.chars().skip(current_index);

            // Put all characters together except the selected one.
            // By leaving the selected one out, it is forgotten and therefore deleted.
            self.input = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.input.chars().count())
    }

    fn reset_cursor(&mut self) {
        self.character_index = 0;
    }
    fn submit_message(&mut self, _message: String) {
        // If fetch successful, update the search list
        // If fetch failed, set the search list to empty
        match fetch_search_result(&_message) {
            Ok(result) => {
                self.search_list.stocks = result;
            }
            Err(_) => {
                self.search_list.stocks = vec![];
            }
        }
        self.input.clear();
        self.reset_cursor();
        self.input_mode = InputMode::Normal;
    }
}
