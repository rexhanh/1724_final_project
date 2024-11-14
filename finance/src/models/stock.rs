pub struct Stock {
    pub name: String,
    pub price: f64,
}

impl Stock {
    pub fn new(name: &str, price: f64) -> Self {
        Self {
            name: name.to_string(),
            price,
        }
    }
}