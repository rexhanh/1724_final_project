use super::stock::Stock;
use ratatui::widgets::ListState;
pub struct StockList {
    pub stocks: Vec<Stock>,
    pub state: ListState,
}

impl StockList {
    pub fn new() -> Self {
        Self {
            stocks: Vec::new(),
            state: ListState::default(),
        }
    }

    pub fn add_stock(&mut self, stock: Stock) {
        self.stocks.push(stock);
    }
}
