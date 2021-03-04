extern crate serde;
use std::{collections::HashMap};
#[derive(serde::Deserialize, Debug)]
pub struct SymbolsResponse {
    pub success : bool,
    pub symbols : HashMap<String, String>
}

#[derive(serde::Deserialize, Debug)]
pub struct RatesResponse {
    pub success: bool,
    pub timestamp: i64,
    pub base: String,
    pub date: String,
    pub rates: HashMap<String, f64>,
}
