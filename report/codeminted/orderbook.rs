pub struct L3Quote
{
    pub id: i64,
    pub price: f64,
    pub size: f64,
}

pub struct OrderBook
{
    pub instrument: String,
    pub bid: BTreeMap<i64, VecDeque<L3Quote>>,
    pub ask: BTreeMap<i64, VecDeque<L3Quote>>,

    price_step: f64,
    price_step_inv: f64,
}

impl OrderBook
{
    pub fn new(price_step: f64) -> OrderBook { ... }
    pub fn update(&mut self, lines: Vec<String>) { ... }
    pub fn from_file(filename: &str) -> Result<OrderBook, String> { ... }
    pub fn print(orderbook: &OrderBook) { ... }
}

impl Iterator for OrderBook 
{
    type Item = Order; 
    fn next(&mut self) -> Option<Self::Item> { ... }
}