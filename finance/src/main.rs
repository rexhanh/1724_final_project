use color_eyre::Result;
use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Layout, Rect},
    style::{Color, Stylize},
    symbols,
    text::Line,
    widgets::{
        Block, Borders, List, ListItem, Padding, Paragraph, StatefulWidget, Widget, Wrap,
    },
    DefaultTerminal,
};

mod models;
use models::stock::Stock;
use models::stock_list::StockList;

mod analytics;
use analytics::render_analytics_screen;

enum Screen {
    Stock,
    Search,
    Analytics,
}
struct App {
    should_quit: bool,
    stock_list: StockList,
    screen: Screen,
    status_message: String,
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    fn new() -> Self {
        let mut stock_list = StockList::new();
        // ! TEST ONLY
        // TODO Fetch the data from the API
        stock_list.add_stock(Stock::new("AAPL", 129.41));
        stock_list.add_stock(Stock::new("GOOGL", 1867.88));
        stock_list.add_stock(Stock::new("MSFT", 244.49));
        stock_list.add_stock(Stock::new("AMZN", 3242.76));
        stock_list.add_stock(Stock::new("TSLA", 672.37));
        Self {
            should_quit: false,
            stock_list,
            screen: Screen::Stock,
            status_message: String::new(), 
        }
    }

    fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        while !self.should_quit {
            terminal.draw(|frame| frame.render_widget(&mut self, frame.area()))?;
            if let Event::Key(key) = event::read()? {
                match self.screen {
                    Screen::Stock => self.handle_stock_screen_key(key),
                    Screen::Search => self.handle_search_screen_key(key),
                    Screen::Analytics => self.handle_analytics_screen_key(key),
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
            KeyCode::Char('a') => {
                if self.stock_list.state.selected().is_some() {
                    // If a stock is selected, go to the analytics screen
                    self.screen = Screen::Analytics;
                } else {
                    // If no stock is selected, set a warning message
                    self.status_message = String::from("Please select a stock before entering analytics.");
                }
            }
            _ => {}
        }
    }
    // Handle keys for Analytics screen
    fn handle_analytics_screen_key(&mut self, key: KeyEvent) {
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

    fn handle_search_screen_key(&mut self, key: KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => self.should_quit = true,
            KeyCode::Backspace | KeyCode::Char('h') => {
                self.screen = Screen::Stock;
            }
            KeyCode::Char('a') => {
                // Display a warning message instead of switching to the analytics screen
                self.status_message = String::from("Cannot enter analytics; you must first select a stock on the stock screen.");
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
            Screen::Analytics => {
                if let Some(i) = self.stock_list.state.selected() {
                    let selected_stock = &self.stock_list.stocks[i];
                    render_analytics_screen(self, selected_stock, area, buf);
                }
            },
        }
    }
}

// Implement for the rendering
impl App {
    /* components */
    fn render_header(area: Rect, buf: &mut Buffer) {
        Paragraph::new("Finance APP").centered().render(area, buf);
    }

    fn render_list(&mut self, area: Rect, buf: &mut Buffer) {
        let block = Block::new().title(Line::raw("Stocks").left_aligned().bold().bg(Color::Cyan));

        let items: Vec<ListItem> = self
            .stock_list
            .stocks
            .iter()
            .map(|stock| ListItem::new(stock.name.clone()))
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
            format!("Name: {}\nPrice: ${:.2}", stock.name, stock.price)
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
        Paragraph::new("Use ↓↑ to move, ← to unselect, s to search, q to quit, h to home, a to analytics")
            .centered()
            .render(area, buf);
    }

    /* screens */
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
        let block = Block::new()
            .title(Line::raw("Search").centered())
            .borders(Borders::ALL)
            .border_set(symbols::border::THICK);

        Paragraph::new("Search goes here...")
            .block(block)
            .render(area, buf);
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let app_result = App::default().run(terminal);
    ratatui::restore();
    app_result
}
