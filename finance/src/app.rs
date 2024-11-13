use color_eyre::Result;
use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Layout, Rect},
    style::{Color, Stylize},
    symbols,
    text::Line,
    widgets::{
        Block, Borders, List, ListItem, ListState, Padding, Paragraph, StatefulWidget, Widget, Wrap,
    },
    DefaultTerminal,
};
use serde::{Deserialize, Serialize};
enum Screen {
    Stock,
    Search,
}
pub struct App {
    should_quit: bool,
    stock_list: StockList,
    screen: Screen,
}
#[derive(Debug, Serialize, Deserialize)]
struct GlobalQuote {
    #[serde(rename = "Global Quote")]
    global_quote: Stock,
}
#[derive(Debug, Serialize, Deserialize)]
struct Stock {
    #[serde(rename = "01. symbol")]
    symbol: String,
    #[serde(rename = "02. open")]
    open: String,
    #[serde(rename = "03. high")]
    high: String,
    #[serde(rename = "04. low")]
    low: String,
    #[serde(rename = "05. price")]
    price: String,
    #[serde(rename = "09. change")]
    change: String,
    #[serde(rename = "10. change percent")]
    change_percent: String,
}

struct StockList {
    stocks: Vec<Stock>,
    state: ListState,
}
const API_KEY: &str = "08GJX8AILBFV6R98";

fn fetch_stock(stock: &str) -> Result<Stock, reqwest::Error> {
    reqwest::blocking::Client::new()
        .get("https://www.alphavantage.co/query")
        .query(&[
            ("function", "GLOBAL_QUOTE"),
            ("symbol", "IBM"),
            ("apikey", "demo"),
        ])
        .send()?
        .json::<GlobalQuote>()
        .map(|body| body.global_quote)
}

impl StockList {
    fn new() -> Self {
        Self {
            stocks: Vec::new(),
            state: ListState::default(),
        }
    }

    fn add_stock(&mut self, stock: Stock) {
        self.stocks.push(stock);
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    fn new() -> Self {
        let stock_list = StockList::new();
        // ! TEST ONLY
        // TODO Fetch the data from the API
        // stock_list.add_stock(Stock::new("AAPL"));
        // stock_list.add_stock(Stock::new("GOOGL"));
        // stock_list.add_stock(Stock::new("MSFT"));
        // stock_list.add_stock(Stock::new("AMZN"));
        // stock_list.add_stock(Stock::new("TSLA"));
        Self {
            should_quit: false,
            stock_list,
            screen: Screen::Stock,
        }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        self.stock_list.add_stock(fetch_stock("TSLA").unwrap());
        while !self.should_quit {
            terminal.draw(|frame| frame.render_widget(&mut self, frame.area()))?;
            if let Event::Key(key) = event::read()? {
                match self.screen {
                    Screen::Stock => self.handle_stock_screen_key(key),
                    Screen::Search => self.handle_search_screen_key(key),
                }
                // self.handle_key(key);
            };
        }
        Ok(())
    }

    fn handle_stock_screen_key(&mut self, key: KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => self.should_quit = true,
            KeyCode::Down => self.select_next(),
            KeyCode::Up => self.select_previous(),
            KeyCode::Left => self.select_none(),
            KeyCode::Char('s') => {
                self.screen = Screen::Search;
            }
            _ => {}
        }
    }

    fn handle_search_screen_key(&mut self, key: KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => self.should_quit = true,
            KeyCode::Backspace | KeyCode::Char('h') => {
                self.screen = Screen::Stock;
            }
            _ => {}
        }
    }

    fn select_next(&mut self) {
        self.stock_list.state.select_next();
    }

    fn select_previous(&mut self) {
        self.stock_list.state.select_previous();
    }

    fn select_none(&mut self) {
        self.stock_list.state.select(None);
    }
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self.screen {
            // TODO Add Screens HERE
            Screen::Stock => self.render_stock_screen(area, buf),
            Screen::Search => self.render_search_screen(area, buf),
        }
    }
}

// Implement for the rendering
impl App {
    fn render_header(area: Rect, buf: &mut Buffer) {
        Paragraph::new("Finance").centered().render(area, buf);
    }

    fn render_list(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::new().title(Line::raw("Stocks").left_aligned().bold().bg(Color::Cyan));

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
                "Name: {}\nPrice: ${}\nOpen: ${}\n",
                stock.symbol, stock.price, stock.open
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
    fn render_chart(&self, area: Rect, buf: &mut Buffer) {
        // TODO Add Chart rendering here
        let block = Block::new()
            .title(Line::raw("Chart").centered())
            .borders(Borders::ALL)
            .border_set(symbols::border::THICK);

        Paragraph::new("Chart goes here...")
            .block(block)
            .render(area, buf);
    }

    fn render_footer(&self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("Use ↓↑ to move, ← to unselect, s to search, q to quit")
            .centered()
            .render(area, buf);
    }

    fn render_stock_screen(&mut self, area: Rect, buf: &mut Buffer) {
        let [header_area, main_area, _footer_area] = Layout::vertical([
            Constraint::Length(2),
            Constraint::Fill(1),
            Constraint::Length(1),
        ])
        .areas(area);

        let [list_area, item_area] =
            Layout::horizontal([Constraint::Percentage(10), Constraint::Percentage(90)])
                .areas(main_area);

        let [_chart_area, info_area] =
            Layout::vertical([Constraint::Percentage(70), Constraint::Percentage(30)])
                .areas(item_area);
        App::render_header(header_area, buf);
        self.render_list(list_area, buf);
        self.render_selected_item(info_area, buf);
        self.render_chart(_chart_area, buf);
        self.render_footer(_footer_area, buf);
    }

    fn render_search_screen(&self, area: Rect, buf: &mut Buffer) {
        let [input_area, main_area, _footer_area] = Layout::vertical([
            Constraint::Fill(1),
            Constraint::Percentage(70),
            Constraint::Fill(1),
        ])
        .areas(area);
        self.render_search_input(input_area, buf);
        self.render_search_result(main_area, buf);
        // let block = Block::new()
        //     .title(Line::raw("Search").centered())
        //     .borders(Borders::ALL)
        //     .border_set(symbols::border::THICK);

        // Paragraph::new("Search goes here...")
        //     .block(block)
        //     .render(main_area, buf);
    }

    fn render_search_input(&self, area: Rect, buf: &mut Buffer) {
        let block = Block::new()
            .title(Line::raw("Search").centered())
            .borders(Borders::ALL)
            .border_set(symbols::border::THICK);

        Paragraph::new("Search goes here...")
            .block(block)
            .render(area, buf);
    }

    fn render_search_result(&self, area: Rect, buf: &mut Buffer) {
        let block = Block::new()
            .title(Line::raw("Result").centered())
            .borders(Borders::ALL)
            .border_set(symbols::border::THICK);

        Paragraph::new("Search goes here...")
            .block(block)
            .render(area, buf);
    }
}
