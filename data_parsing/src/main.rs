mod orderbook;

use crate::orderbook::{OrderBook};

fn lines_from_string(s: &str) -> Vec<String>
{
    s.split('\n').map(str::to_string).collect()
}

fn main()
{
    let mut res: OrderBook = OrderBook::from_json("data/test.txt").unwrap();
    OrderBook::print(&res);

    let string = "{\"date\":\"2022-11-10T07:00:03.124000004\",\"instrument\":\"USD/RUB_T+1\",\"type\":\"INCREMENT\",\"side\":\"BID\",\"quotes\":{\"added\":[],\"changed\":[],\"deleted\":[6153236882637504]}}\n{\"date\":\"2022-11-10T07:00:03.124000004\",\"instrument\":\"USD/RUB_T+1\",\"type\":\"INCREMENT\",\"side\":\"ASK\",\"quotes\":{\"added\":[{\"id\":1116150503,\"price\":161.505,\"size\":500000.0},{\"id\":16123234102,\"price\":161.11,\"size\":10000.0},{\"id\":161554102,\"price\":161.11,\"size\":30.0},{\"id\":1617685102,\"price\":161.11,\"size\":50000.0}],\"changed\":[{\"id\":6151503,\"price\":61.515,\"size\":56000.0}],\"deleted\":[6151503,615102,61507504,6150503]}}";
    let lines = lines_from_string(string);
    res.update(lines).unwrap();
    OrderBook::print(&res);
}
