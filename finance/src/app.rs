use color_eyre::Result;
use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Layout, Position, Rect},
    style::{Color, Stylize},
    symbols,
    text::Line,
    widgets::{
        Block, Borders, List, ListItem, ListState, Padding, Paragraph, StatefulWidget, Widget, Wrap,
    },
    DefaultTerminal, Frame,
};
mod model;
pub use model::{App, InputMode, Quote, Screen, SearchList, StockList};
mod utils;
pub use utils::{fetch_search_result, fetch_stock};

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
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    fn new() -> Self {
        let stock_list = StockList::new();
        let search_list = SearchList::new();
        Self {
            should_quit: false,
            stock_list,
            search_list,
            screen: Screen::Stock,
            input_mode: InputMode::Normal,
            input: String::new(),
            character_index: 0,
        }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        while !self.should_quit {
            terminal.draw(|frame| self.draw(frame))?;
            if let Event::Key(key) = event::read()? {
                match self.screen {
                    Screen::Stock => self.handle_stock_screen_key(key),
                    Screen::Search => self.handle_search_screen_key(key),
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
        match self.input_mode {
            InputMode::Normal => match key.code {
                KeyCode::Char('q') | KeyCode::Esc => self.should_quit = true,
                KeyCode::Char('s') => {
                    self.screen = Screen::Stock;
                }
                KeyCode::Char('i') => {
                    self.input_mode = InputMode::Editing;
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

    fn select_next(&mut self) {
        self.stock_list.state.select_next();
    }

    fn select_previous(&mut self) {
        self.stock_list.state.select_previous();
    }

    fn select_none(&mut self) {
        self.stock_list.state.select(None);
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
    fn add_stock(&mut self) {
        if let Some(i) = self.search_list.state.selected() {
            let stock_symbol = self.search_list.stocks[i].clone().symbol;
            let stock = fetch_stock(&stock_symbol);
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
        }
    }

    fn draw_stock_screen(&mut self, frame: &mut Frame) {
        let [header_area, main_area, _footer_area] = Layout::vertical([
            Constraint::Length(2),
            Constraint::Fill(1),
            Constraint::Length(1),
        ])
        .areas(frame.area());

        let [list_area, item_area] =
            Layout::horizontal([Constraint::Percentage(10), Constraint::Percentage(90)])
                .areas(main_area);

        let [_chart_area, info_area] =
            Layout::vertical([Constraint::Percentage(70), Constraint::Percentage(30)])
                .areas(item_area);
        App::render_header(header_area, frame.buffer_mut());
        self.render_list(list_area, frame.buffer_mut());
        self.render_selected_item(info_area, frame.buffer_mut());
        self.render_chart(_chart_area, frame.buffer_mut());
        self.render_footer(_footer_area, frame.buffer_mut());
    }

    fn draw_search_screen(&mut self, frame: &mut Frame) {
        let [input_area, main_area, _footer_area] = Layout::vertical([
            Constraint::Fill(1),
            Constraint::Percentage(70),
            Constraint::Fill(1),
        ])
        .areas(frame.area());
        // self.render_search_input(input_area, frame.buffer_mut());
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
}

// Implementaation for each of the rendering
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
