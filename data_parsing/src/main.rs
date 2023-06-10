mod orderbook;

use crate::orderbook::{OrderBook};

fn main()
{
    OrderBook::calculate_features("data/USD_RUB_T+1__2022-11-10", "USD/RUB_T+1".to_string(), 0.0025);
}