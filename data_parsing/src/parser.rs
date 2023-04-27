use serde_json::{
    Value,
};
use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};
use std::collections::{VecDeque};
use crate::orderbook::{new, OrderBook, L3Quote,};

fn read_lines<P>(filename: &P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
}

pub fn read_json(filename: &str) -> Result<OrderBook, String>
{
    let mut orderbook = OrderBook { bid: new(), ask: new() };

    if let Ok(lines) = read_lines(&filename)
    {
        for line in lines
        {
            if let Ok(line_value) = line
            {
                let json_line: Value = serde_json::from_str(&line_value).unwrap();

                let json_line_side= &json_line["side"].to_string()[1..4];
                let side= if json_line_side == "BID" { &mut orderbook.bid } else { &mut orderbook.ask };

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
    orderbook.ask.retain(|_k,v| v.len() > 0);
    orderbook.bid.retain(|_k,v| v.len() > 0);

    Ok(orderbook)
}
