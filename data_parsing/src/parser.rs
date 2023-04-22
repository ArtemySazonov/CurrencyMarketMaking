use serde_json::{
    Value,
};
use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

use crate::orderbook::{new, Order, OrderBook, Quote, QuoteElement,};

fn read_lines<P>(filename: &P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
}

pub fn read_json(filename: &str) -> std::result::Result<OrderBook, std::string::String>
{
    let mut orderbook = OrderBook
    {
        _data: new()
    };
    let _bid_text: std::string::String = "BID".to_string();
    let _ask_text: std::string::String = "ASK".parse().unwrap();
    let _inc_text: std::string::String = "INCREMENT".parse().unwrap();

    if let Ok(lines) = read_lines(&filename)
    {
        for line in lines
        {
            if let Ok(line_value) = line
            {
                let json_line: Value = serde_json::from_str(&line_value).unwrap();

                let json_line_date = json_line["date"].to_string();
                let json_line_side= json_line["side"].to_string();
                let json_line_instrument = json_line["instrument"].to_string();
                let json_line_type = json_line["type"].to_string();

                let json_line_quotes = &json_line["quotes"];

                let mut json_line_quotes_added_vec   = Vec::new();
                let mut json_line_quotes_changed_vec = Vec::new();
                let mut json_line_quotes_deleted_vec = Vec::new();

                let json_line_quotes_added = json_line_quotes["added"].as_array();
                let json_line_quotes_changed = json_line_quotes["changed"].as_array();
                let json_line_quotes_deleted = json_line_quotes["deleted"].as_array();

                if json_line_quotes_added != None
                {
                    let json_line_quotes_added = json_line_quotes_added.unwrap();
                    for quote in json_line_quotes_added
                    {
                        let id = quote["id"].as_i64().unwrap();
                        let price = quote["price"].as_f64().unwrap();
                        let size = quote["size"].as_f64().unwrap();

                        json_line_quotes_added_vec.push(QuoteElement{
                            _id:    id,
                            _price: price,
                            _size:  size,
                        })
                    }
                }
                if json_line_quotes_changed != None
                {
                    let json_line_quotes_changed  = json_line_quotes_changed.unwrap();
                    for quote in json_line_quotes_changed
                    {
                        let id = quote["id"].as_i64().unwrap();
                        let price = quote["price"].as_f64().unwrap();
                        let size = quote["size"].as_f64().unwrap();

                        json_line_quotes_changed_vec.push(QuoteElement{
                            _id:    id,
                            _price: price,
                            _size:  size,
                        })
                    }
                }
                if json_line_quotes_deleted != None
                {
                    let json_line_quotes_deleted = json_line_quotes_deleted.unwrap();
                    for quote in json_line_quotes_deleted
                    {
                        let id: i64 = quote.as_i64().unwrap();
                        json_line_quotes_deleted_vec.push(id)
                    }
                }

                let ord = Order
                {
                    _date:       json_line_date,
                    _side:       json_line_side,
                    _instrument: json_line_instrument,
                    _type:       json_line_type,
                    _quotes:     Quote
                                 {
                                     _added:   json_line_quotes_added_vec,
                                     _changed: json_line_quotes_changed_vec,
                                     _deleted: json_line_quotes_deleted_vec,
                                 },
                };
                orderbook.insert(ord)
            }
        }
    }
    else
    {
        return Err("File could not be opened.".to_string());
    }
    Ok(orderbook)
}
