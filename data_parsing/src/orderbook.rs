use std::collections::{BTreeMap, VecDeque,};
use serde_json::{Value,};
use std::{fs::File, io::{self, BufRead}, path::Path, };
use kdam::tqdm;

pub struct L3Quote
{
    pub id: i64,
    pub price: f64,
    pub size: f64,
}

pub struct OrderBook // key = price
{
    pub instrument: String,
    pub bid: BTreeMap<String, VecDeque<L3Quote>>,
    pub ask: BTreeMap<String, VecDeque<L3Quote>>,
}

impl OrderBook
{
    pub fn new_side() -> BTreeMap<String, VecDeque<L3Quote>>
    {
        BTreeMap::new()
    }

    fn read_lines<P>(filename: &P) -> io::Result<io::Lines<io::BufReader<File>>>
        where P: AsRef<Path>, {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }

    pub fn update(&mut self, string: String) -> Result<&mut OrderBook, String>
    {
        let lines = string.lines();
        for line in tqdm!(lines)
        {
            let json_line: Value = serde_json::from_str(&line).unwrap();

            let json_line_side = &json_line["side"].to_string()[1..4];
            let side = if json_line_side == "BID" { &mut self.bid } else { &mut self.ask };
            if self.instrument != json_line["instrument"].to_string() { return Err("Wrong instrument".to_string()); }

            let json_line_quotes = &json_line["quotes"];

            let json_line_quotes_vec = json_line_quotes["added"].as_array();
            if json_line_quotes_vec != None
            {
                let json_line_quotes_added = json_line_quotes_vec.unwrap();
                for quote in json_line_quotes_added
                {
                    let id = quote["id"].as_i64().unwrap();
                    let price = quote["price"].as_f64().unwrap();
                    let price_key = price.to_string();
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
                    let price_key = price.to_string();
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
        let mut orderbook = OrderBook {instrument: String::from(""), bid: OrderBook::new_side(), ask: OrderBook::new_side() };

        if let Ok(lines) = OrderBook::read_lines(&filename)
        {
            for line in tqdm!(lines)
            {
                if let Ok(line_value) = line
                {
                    let json_line: Value = serde_json::from_str(&line_value).unwrap();

                    let json_line_side= &json_line["side"].to_string()[1..4];
                    let side= if json_line_side == "BID" { &mut orderbook.bid } else { &mut orderbook.ask };
                    orderbook.instrument = json_line["instrument"].to_string();
                    //let json_line_type = json_line["type"].to_string();

                    let json_line_quotes = &json_line["quotes"];

                    let json_line_quotes_vec = json_line_quotes["added"].as_array();
                    if json_line_quotes_vec != None
                    {
                        let json_line_quotes_added = json_line_quotes_vec.unwrap();
                        for quote in json_line_quotes_added
                        {
                            let id = quote["id"].as_i64().unwrap();
                            let price = quote["price"].as_f64().unwrap();
                            let price_key = price.to_string();
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
                            let price_key = price.to_string();
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
            }
        }
        else
        {
            return Err("File could not be opened.".to_string());
        }
        orderbook.ask.retain(|_k,v| v.len() != 0);
        orderbook.bid.retain(|_k,v| v.len() != 0);

        Ok(orderbook)
    }

    pub fn print(orderbook: &OrderBook)
    {
        println!("\n\nAsk:");
        for (key, value) in &orderbook.ask
        {
            let price_level: f64 = key.parse().unwrap();
            print!("\nRUB {:.4}: ", price_level);
            for quote in value {
                print!("(id: {}, size: {}), ", quote.id, quote.size);
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