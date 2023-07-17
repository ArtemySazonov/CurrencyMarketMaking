mod orderbook;

use crate::orderbook::{OrderBook};

fn main()
{
    let filenames_rub = ["data/USD_RUB_T+1__2022-10-24", "data/USD_RUB_T+1__2022-10-31", "data/USD_RUB_T+1__2022-11-03", "data/USD_RUB_T+1__2022-11-10"];
    let filenames_chn = ["data/USD_CNH_T+1__2022-10-04", "data/USD_CNH_T+1__2022-10-12", "data/USD_CNH_T+1__2022-10-21", "data/USD_CNH_T+1__2022-10-31"];

    for filename in filenames_rub
    {
        OrderBook::calculate_features(filename, "USD/RUB_T+1".to_string(), 0.0025);
        println!("Completed {}!", filename);
    }
    for filename in filenames_chn
    {
        OrderBook::calculate_features(filename, "USD/CNH_T+1".to_string(), 0.0025);
        println!("Completed {}!", filename);
    }
}