mod orderbook;

use crate::orderbook::{OrderBook};

fn main()
{
    let res = OrderBook::from_file("data/USD_RUB_T+1__2022-11-10", "USD/RUB_T+1".to_string(), 0.0025);
    res.print();
}