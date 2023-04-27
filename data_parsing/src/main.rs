mod orderbook;

use crate::orderbook::{OrderBook};

fn main()
{
    let mut res: OrderBook = OrderBook::from_json("data/test.txt").unwrap();
    OrderBook::print(&res);
    let string = "{\"date\":\"2022-11-10T07:00:03.124000001\",\"instrument\":\"USD/RUB_T+1\",\"type\":\"INCREMENT\",\"side\":\"ASK\",\"quotes\":{\"added\":[{\"id\":1507504,\"price\":91.5075,\"size\":70000.0}],\"changed\":[],\"deleted\":[11512504]}}\n{\"date\":\"2022-11-10T07:00:03.124000004\",\"instrument\":\"USD/RUB_T+1\",\"type\":\"INCREMENT\",\"side\":\"BID\",\"quotes\":{\"added\":[],\"changed\":[],\"deleted\":[6153236882637504]}}\n{\"date\":\"2022-11-10T07:00:03.124000004\",\"instrument\":\"USD/RUB_T+1\",\"type\":\"INCREMENT\",\"side\":\"ASK\",\"quotes\":{\"added\":[{\"id\":1116150503,\"price\":161.505,\"size\":500000.0},{\"id\":16123234102,\"price\":161.11,\"size\":50000.0},{\"id\":161554102,\"price\":161.11,\"size\":50000.0},{\"id\":1617685102,\"price\":161.11,\"size\":50000.0}],\"changed\":[{\"id\":6151503,\"price\":61.515,\"size\":56000.0}],\"deleted\":[6151503,615102,61507504,6150503]}}".to_string();
    res.update(string).unwrap();
    OrderBook::print(&res);
}
