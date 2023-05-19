mod orderbook;

use crate::orderbook::{OrderBook};


fn main()
{
    let res = OrderBook::from_file("data/test.txt", 0.0025).unwrap();
    res.print();
}