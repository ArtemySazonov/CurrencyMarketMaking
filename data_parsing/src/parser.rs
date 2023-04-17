enum Side
{
    BID,
    ASK,
}
enum Type
{
    INCREMENT,
    VALUE,
}

pub struct QuoteElement
{
    _id: i32,
    _price: f32,
    _size: i32,
}

pub struct Quote
{
    _added: Box<QuoteElement>,
    _changed: Box<QuoteElement>,
    _deleted: Box<i32>,
}

pub struct Order
{
    _date: String,
    _instrument: String,
    _type: Type,
    _side: Side,
    _quotes: Box<Quote>,
}

pub struct OrderBook
{
    data: Box<Order>,
}

fn read_json()
{

}