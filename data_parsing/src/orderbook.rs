use std::collections::{BTreeMap, VecDeque};

#[derive(Debug)]
pub struct L3Quote
{
    pub(crate) id: i64,
    pub(crate) price: f64,
    pub(crate) size: f64,
}

pub struct OrderBook // key = price
{
    pub(crate) bid: BTreeMap<String, VecDeque<L3Quote>>,
    pub(crate) ask: BTreeMap<String, VecDeque<L3Quote>>,
}

pub fn new() -> BTreeMap<String, VecDeque<L3Quote>>
{
    BTreeMap::new()
}

impl OrderBook
{
    pub fn print(orderbook: &OrderBook)
    {
        println!("\n\nAsk:");
        for (key, value) in &orderbook.ask
        {
            let price_level: f64 = key.parse().unwrap();
            print!("\nRUB {:.4}: ", price_level);
            for quote in value {
                print!("(id: {}, size: {})", quote.id, quote.size);
            }
        }

        println!("\n\nBid:");
        for (key, value) in &orderbook.bid
        {
            let price_level: f64 = key.parse().unwrap();
            print!("\nRUB {:.4}: ", price_level);
            for quote in value {
                print!("(id: {}, size: {}), ", quote.id, quote.size);
            }
        }
        println!("\n");
    }
}