use std::collections::{BTreeMap, VecDeque,};
use serde_json::{Value,};
use std::{fs::File, io::{self, BufRead, BufReader}, path::Path, };

pub struct L3Quote
{
    pub id: i64,
    pub price: f64,
    pub size: f64,
}

pub struct OrderBook // key = price/0.0025
{
    pub instrument: String,
    pub bid: BTreeMap<i64, VecDeque<L3Quote>>,
    pub ask: BTreeMap<i64, VecDeque<L3Quote>>,
}

impl OrderBook
{
    fn new_side() -> BTreeMap<i64, VecDeque<L3Quote>>
    {
        BTreeMap::new()
    }

    pub fn new() -> OrderBook
    {
        OrderBook{ instrument: "".to_string(), bid: OrderBook::new_side(), ask: OrderBook::new_side() }
    }

    fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
        BufReader::new(File::open(filename)?).lines().collect()
    }

    pub fn update(&mut self, lines: Vec<String>) -> Result<&mut OrderBook, String>
    {
        for line in lines
        {
            let json_line: Value = serde_json::from_str(&line).unwrap();

            let json_line_side = &json_line["side"].to_string()[1..4];
            let side = if json_line_side == "BID" { &mut self.bid } else { &mut self.ask };
            if self.instrument != "" && self.instrument != json_line["instrument"].to_string()
            {
                return Err("Wrong instrument".to_string());
            }

            let json_line_quotes = &json_line["quotes"];

            let json_line_quotes_vec = json_line_quotes["added"].as_array();
            if json_line_quotes_vec != None
            {
                let json_line_quotes_added = json_line_quotes_vec.unwrap();
                for quote in json_line_quotes_added
                {
                    let id = quote["id"].as_i64().unwrap();
                    let price = quote["price"].as_f64().unwrap();
                    let price_key = (price*400.0) as i64;
                    let size = quote["size"].as_f64().unwrap();
                    side.entry(price_key).or_insert(VecDeque::new()).push_back(L3Quote {
                        id,
                        price,
                        size,
                    });
                }
            }

            let json_line_quotes_vec = json_line_quotes["changed"].as_array();
            if json_line_quotes_vec != None
            {
                let json_line_quotes_changed = json_line_quotes_vec.unwrap();
                for quote in json_line_quotes_changed
                {
                    let id = quote["id"].as_i64().unwrap();
                    let price = quote["price"].as_f64().unwrap();
                    let price_key = (price*400.0) as i64;
                    let size = quote["size"].as_f64().unwrap();
                    for (_key, value) in &mut *side
                    {
                        value.retain(|x| x.id != id)
                    }
                    side.entry(price_key).or_insert(VecDeque::new()).push_back(L3Quote {
                        id,
                        price,
                        size,
                    });
                }
            }

            let json_line_quotes_vec = json_line_quotes["deleted"].as_array();
            if json_line_quotes_vec != None
            {
                let json_line_quotes_deleted = json_line_quotes_vec.unwrap();
                for quote in json_line_quotes_deleted
                {
                    let id = quote.as_i64().unwrap();
                    for (_key, value) in &mut *side
                    {
                        value.retain(|x| x.id != id);
                    }
                }
            }
        }
        self.ask.retain(|_k,v| v.len() != 0);
        self.bid.retain(|_k,v| v.len() != 0);

        Ok(self)
    }

    pub fn from_json(filename: &str) -> Result<OrderBook, String>
    {
        let mut orderbook = OrderBook::new();

        if let Ok(lines) = OrderBook::lines_from_file(&filename)
        {
            orderbook.update(lines).expect("TODO: panic message");
        }
        else
        {
            return Err("File could not be opened.".to_string());
        }

        Ok(orderbook)
    }

    pub fn print(orderbook: &OrderBook)
    {
        println!("\n\nAsk:");
        for (key, value) in &orderbook.ask
        {
            let price_level: f64 = (*key as f64) * 0.0025;
            print!("\nRUB {:.4}: ", price_level);
            for quote in value {
                print!("(id: {}, size: {}), ", quote.id, quote.size);
            }
        }

        println!("\n\nBid:");
        for (key, value) in &orderbook.bid
        {
            let price_level: f64 = (*key as f64) * 0.0025;
            print!("\nRUB {:.4}: ", price_level);
            for quote in value {
                print!("(id: {}, size: {}), ", quote.id, quote.size);
            }
        }
        println!("\n");
    }
}