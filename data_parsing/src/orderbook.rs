use std::collections::BTreeMap;

pub struct QuoteElement
{
    pub(crate) _id: i64,
    pub(crate) _price: f64,
    pub(crate) _size: f64,
}

pub struct Quote
{
    pub(crate) _added: Vec<QuoteElement>,
    pub(crate) _changed: Vec<QuoteElement>,
    pub(crate) _deleted: Vec<i64>,
}

pub struct Order
{
    pub(crate) _date: String,
    pub(crate) _side: String,
    pub(crate) _instrument: String,
    pub(crate) _type: String,
    pub(crate) _quotes: Quote,
}

pub struct OrderBook // key = date-time
{
    pub(crate) _data: BTreeMap<String, Order>,
}

pub fn new() -> BTreeMap<String, Order>
{
    BTreeMap::new()
}

impl OrderBook
{
    pub fn insert(&mut self, ord: Order)
    {
        let key: &String = &ord._date.clone();
        self._data.insert(key.to_string(), ord);
    }

    pub(crate) fn print(book: &OrderBook)
    {
        let mut i = 0;
        for (&ref key, &ref order) in &book._data {
            i+=1;
            let side = &order._side;
            let instrument = &order._instrument;
            let _type = &order._type;
            print!("\n{side} on {instrument} was placed at {key}. \nOrder details ({_type}):\n\tAdded: ");
            for &ref quote in &order._quotes._added
            {
                let id = quote._id;
                let price = quote._price;
                let size = quote._size;
                print!("(id:{id}; price:{price}; size:{size}), ");
            }
            print!("\n\tChanged: ");
            for &ref quote in &order._quotes._changed
            {
                let id = quote._id;
                let price = quote._price;
                let size = quote._size;
                print!("(id:{id}; price:{price}; size:{size}), ");
            }
            print!("\n\tDeleted: ");
            for &ref quote in &order._quotes._deleted
            {
                print!("(id:{quote}), ");
            }
            print!("\n");
            if i == 20 { break; }
        }
        println!();
    }
}