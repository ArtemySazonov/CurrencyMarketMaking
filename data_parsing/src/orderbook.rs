use std::{io::BufReader, path::Path, fs::File, io::BufRead, io::Error, collections::{BTreeMap, VecDeque,}};
use serde::{Deserialize, Serialize};

const BAD_STRING:          &str = "Bad string found. Check the inputs.";
const BAD_PATH_TO_FILE:    &str = "File could not be found. Check the inputs.";
const INSTRUMENT_MISMATCH: &str = "Instrument mismatch. Check the inputs.";

pub enum Side
{
    BID,
    ASK,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct L3Quote
{
    pub id: i64,
    pub price: f64,
    pub size: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Quotes
{
    pub added: Vec<L3Quote>,
    pub changed: Vec<L3Quote>,
    pub deleted: Vec<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum QuotesEnum {
    Quotes(Quotes),
    Vec(Vec<String>)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tick
{
    pub date: String,
    pub instrument: String,
    pub r#type: String,
    pub side: String,
    pub quotes: QuotesEnum,
}

#[derive(Debug, PartialEq)]
pub struct OrderBook
{
    pub instrument: String,
    pub date: String,

    pub bid: BTreeMap<i64, VecDeque<L3Quote>>,
    pub ask: BTreeMap<i64, VecDeque<L3Quote>>,

    price_step: f64,
    price_step_inv: f64,
}

impl OrderBook
{
    fn new_side() -> BTreeMap<i64, VecDeque<L3Quote>>
    {
        BTreeMap::new()
    }

    pub fn new(price_step: f64) -> OrderBook
    {
        OrderBook{ instrument: "".to_string(), date: "".to_string(), bid: OrderBook::new_side(), ask: OrderBook::new_side(), price_step, price_step_inv: 1./price_step }
    }

    pub fn update(&mut self, line: &String) -> Result<&mut OrderBook, String>
    {
        let json_line: Tick = serde_json::from_str(&line).unwrap();

        let side = if json_line.side == String::from("BID") { &mut self.bid } else { &mut self.ask };
        if self.instrument == json_line.instrument || self.instrument == ""
        {
            if self.instrument == ""
            {
                self.instrument = json_line.instrument.to_string();
            }
            self.date = json_line.date;

            let quotes = match json_line.quotes
            {
                QuotesEnum::Quotes(quotes) => quotes,
                QuotesEnum::Vec(_) => {
                    return Ok(self);
                },
            };
            for quote in quotes.added
            {
                let price_key = (quote.price * self.price_step_inv) as i64;
                side.entry(price_key).or_insert(VecDeque::new()).push_back(L3Quote {
                    id: quote.id,
                    price: quote.price,
                    size: quote.size,
                });
            }

            for quote in quotes.changed
            {
                let price_key = (quote.price * self.price_step_inv) as i64;
                for (_key, value) in &mut *side
                {
                    value.retain(|x| x.id != quote.id)
                }
                side.entry(price_key).or_insert(VecDeque::new()).push_back(L3Quote {
                    id: quote.id,
                    price: quote.price,
                    size: quote.size,
                });
            }

            for id in quotes.deleted
            {
                for (_key, value) in &mut *side
                {
                    value.retain(|x| x.id != id);
                }
            }
        }
        else
        {
            return Err(INSTRUMENT_MISMATCH.to_string());
        }

        Ok(self)
    }

    fn clean(&mut self)
    {
        self.ask.retain(|_k,v| v.len() != 0);
        self.bid.retain(|_k,v| v.len() != 0);
    }

    pub fn update_from_string(&mut self, lines: String) -> Result<&mut OrderBook, String>
    {
        let lines: Vec<String> = lines.split("\n").map(|x| x.to_string()).collect();
        for line in &lines
        {
            self.update(line).expect(BAD_STRING);
        }
        self.clean();
        Ok(self)
    }

    pub fn from_file(filename: &str, price_step: f64) -> Result<OrderBook, String>
    {
        let mut orderbook = OrderBook::new(price_step);
        let path = Path::new(filename);
        let display = path.display();

        let file = match File::open(&path) {
            Err(why) => panic!("couldn't open {}: {}", display, Error::to_string(&why)),
            Ok(file) => file,
        };
        let reader = BufReader::new(file);
        let lines = reader.lines();

        for result in lines
        {
            if let Ok(line) = result
            {
                orderbook.update(&line).expect(BAD_PATH_TO_FILE);
            }
        }
        orderbook.clean();
        Ok(orderbook)
    }

    pub fn from_str(json: &str, price_step: f64) -> Result<OrderBook, String>
    {
        let mut orderbook = OrderBook::new(price_step);
        let lines: Vec<String> = json.split("\n").map(|x| x.to_string()).collect();

        for line in &lines
        {
            orderbook.update(line).expect(BAD_STRING);
        }
        orderbook.clean();
        Ok(orderbook)
    }

    pub fn print_side(&self, _side: Side)
    {
        let side = match _side {
            Side::ASK => &self.ask,
            Side::BID => &self.bid,
        };
        println!("{}:", match _side {
            Side::ASK => "ASK",
            Side::BID => "BID",
        });
        for (key, value) in side
        {
            let price_level: f64 = (*key as f64) * self.price_step;
            print!("RUB {:.4}: ", price_level);
            for quote in value {
                print!("(id: {}, size: {}), ", quote.id, quote.size);
            }
            print!("\n");
        }
        print!("\n");
    }

    pub fn print(&self)
    {
        println!("Order book for {} at {}", self.instrument, self.date);
        self.print_side(Side::BID);
        self.print_side(Side::ASK);
        print!("\n");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn orderbook_create_from_str()
    {
        /*
         * This test checks that the orderbook is created correctly
         */
        let price_step: f64 = 0.0025;
        let price_step_inv: f64 = 400.0;

        let info: String = format!("{}\n{}", r#"{"instrument":"BTCUSD","type":"INCREMENT","date":"2019-01-01T00:00:00.000Z","side":"BID","quotes":{"added":[{"id":1,"price":4000.0,"size":1.05}],"changed":[],"deleted":[]}}"#, r#"{"instrument":"BTCUSD","type":"INCREMENT","date":"2019-01-01T00:00:00.000Z","side":"ASK","quotes":{"added":[{"id":2,"price":3000.0,"size":1.10}],"changed":[],"deleted":[]}}"#);
        let info: &str = &info;

        let mut bid_expected: BTreeMap<i64, VecDeque<L3Quote>> = BTreeMap::new();
        let mut ask_expected: BTreeMap<i64, VecDeque<L3Quote>> = BTreeMap::new();

        bid_expected.insert((4000.0*price_step_inv) as i64, vec![L3Quote {
            id: 1,
            price: 4000.0,
            size: 1.05,
        }].into_iter().collect());

        ask_expected.insert((3000.0*price_step_inv) as i64, vec![L3Quote {
            id: 2,
            price: 3000.0,
            size: 1.10,
        }].into_iter().collect());

        let orderbook = OrderBook::from_str(info, price_step).expect("Failed to create orderbook");

        assert_eq!(orderbook.instrument, "BTCUSD".to_string());
        assert_eq!(orderbook.price_step, price_step);
        assert_eq!(orderbook.price_step_inv, price_step_inv);
        assert_eq!(orderbook.date, "2019-01-01T00:00:00.000Z".to_string());
        assert_eq!(orderbook.bid, bid_expected);
        assert_eq!(orderbook.ask, ask_expected);
    }

    #[test]
    fn orderbook_update_from_str()
    {
        /*
         * This test checks that the orderbook is updated correctly
         */
        let price_step: f64 = 0.0025;
        let price_step_inv: f64 = 400.0;

        let mut bid_expected: BTreeMap<i64, VecDeque<L3Quote>> = BTreeMap::new();
        let mut ask_expected: BTreeMap<i64, VecDeque<L3Quote>> = BTreeMap::new();
        let mut orderbook = OrderBook::new(price_step);

        let info1: &str = r#"{"instrument":"BTCUSD","type":"INCREMENT","date":"2019-01-01T00:00:00.000Z","side":"BID","quotes":{"added":[{"id":1,"price":4001.0,"size":1.05}],"changed":[],"deleted":[]}}"#;
        let info2: &str = r#"{"instrument":"BTCUSD","type":"INCREMENT","date":"2019-01-01T00:00:00.000Z","side":"ASK","quotes":{"added":[{"id":2,"price":4002.0,"size":1.10}],"changed":[],"deleted":[]}}"#;

        bid_expected.insert((4001.0*price_step_inv) as i64, VecDeque::from([L3Quote{id: 1, price: 4001.0, size: 1.05}]));
        ask_expected.insert((4002.0*price_step_inv) as i64, VecDeque::from([L3Quote{id: 2, price: 4002.0, size: 1.10}]));

        orderbook.update_from_string(info1.to_string()).expect(BAD_STRING);
        orderbook.update_from_string(info2.to_string()).expect(BAD_STRING);

        assert_eq!(orderbook.instrument, "BTCUSD".to_string());
        assert_eq!(orderbook.price_step, price_step);
        assert_eq!(orderbook.price_step_inv, price_step_inv);
        assert_eq!(orderbook.date, "2019-01-01T00:00:00.000Z".to_string());
        assert_eq!(orderbook.bid, bid_expected);
        assert_eq!(orderbook.ask, ask_expected);

        let info3: &str = r#"{"instrument":"BTCUSD","type":"INCREMENT","date":"2019-01-01T00:00:00.000Z","side":"BID","quotes":{"added":[{"id":3,"price":4003.0,"size":1.05}],"changed":[],"deleted":[]}}"#;
        let info4: &str = r#"{"instrument":"BTCUSD","type":"INCREMENT","date":"2019-01-01T00:00:00.000Z","side":"ASK","quotes":{"added":[{"id":4,"price":4004.0,"size":1.10}],"changed":[],"deleted":[]}}"#;

        bid_expected.insert((4003.0*price_step_inv) as i64, VecDeque::from([L3Quote{id: 3, price: 4003.0, size: 1.05}]));
        ask_expected.insert((4004.0*price_step_inv) as i64, VecDeque::from([L3Quote{id: 4, price: 4004.0, size: 1.10}]));

        orderbook.update_from_string(info3.to_string()).expect(BAD_STRING);
        orderbook.update_from_string(info4.to_string()).expect(BAD_STRING);

        assert_eq!(orderbook.instrument, "BTCUSD".to_string());
        assert_eq!(orderbook.price_step, price_step);
        assert_eq!(orderbook.price_step_inv, price_step_inv);
        assert_eq!(orderbook.date, "2019-01-01T00:00:00.000Z".to_string());
        assert_eq!(orderbook.bid, bid_expected);
        assert_eq!(orderbook.ask, ask_expected);

        let info5: &str = r#"{"instrument":"BTCUSD","type":"INCREMENT","date":"2019-01-01T00:00:00.000Z","side":"BID","quotes":{"added":[],"changed":[{"id":3,"price":4003.0,"size":1.06}],"deleted":[]}}"#;
        let info6: &str = r#"{"instrument":"BTCUSD","type":"INCREMENT","date":"2019-01-01T00:00:00.000Z","side":"ASK","quotes":{"added":[],"changed":[{"id":4,"price":4004.0,"size":1.11}],"deleted":[]}}"#;

        bid_expected.entry((4003.0*price_step_inv) as i64).and_modify(|v| v[0].size = 1.06);
        ask_expected.entry((4004.0*price_step_inv) as i64).and_modify(|v| v[0].size = 1.11);

        orderbook.update_from_string(info5.to_string()).expect(BAD_STRING);
        orderbook.update_from_string(info6.to_string()).expect(BAD_STRING);

        assert_eq!(orderbook.instrument, "BTCUSD".to_string());
        assert_eq!(orderbook.price_step, price_step);
        assert_eq!(orderbook.price_step_inv, price_step_inv);
        assert_eq!(orderbook.date, "2019-01-01T00:00:00.000Z".to_string());
        assert_eq!(orderbook.bid, bid_expected);
        assert_eq!(orderbook.ask, ask_expected);
    }

    #[test]
    fn orderbook_create_non_increment()
    {
        /*
         * This test checks that the orderbook is created correctly from a non-incremental message (SNAPSHOT)
         */
        let info: &str = r#"{"instrument":"BTCUSD","type":"SNAPSHOT","date":"2019-01-01T00:00:00.000Z","side":"BID","quotes":[]}"#;
        let _orderbook = OrderBook::from_str(info, 0.0025).expect("Failed to create orderbook from string");
    }

    #[test]
    #[should_panic]
    fn orderbook_create_from_incorrect_json()
    {
        /*
         * This test should panic since 'instrument' is misspelled as 'instrumeent'
         */
        let price_step: f64 = 0.0025;
        let info: &str = r#"{"instrumeent":"BTCUSD","type":"INCREMENT","date":"2019-01-01T00:00:00.000Z","side":"BID","quotes":{"added":[{"id":1,"price":4000.0,"size":1.05}],"changed":[],"deleted":[]}}"#;
        let _orderbook = OrderBook::from_str(info, price_step).expect("Failed to create orderbook from string");
    }
}
