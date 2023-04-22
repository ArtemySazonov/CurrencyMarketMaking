/*
 * Создать структуру данных в rust, отражающую структуру данных JSON, навесить аннотации
 * serde-rs/json, считывать в эту структуру текстовый файл с сырыми данными, 10 первых
 * вывести на экран (уже распарсенных в структуре). Написать юнит-тест для проверки
 * корректности считывания файла.
 */

mod orderbook;
mod parser;

use crate::orderbook::{OrderBook};
use crate::parser::read_json;

fn main()
{
    let path = "data/test.txt";
    let _res: OrderBook = read_json(&path).unwrap();
    OrderBook::print(&_res);
}
