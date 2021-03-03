extern crate serde;
use serde::Deserialize;
use std::{collections::HashMap};

#[derive(Debug)]
pub struct Symbol {
    pub code: String,
    pub name: String
}
#[derive(Deserialize, Debug)]
pub struct SymbolsResponse {
    pub success : bool,
    pub symbols : HashMap<String, String>
}

#[derive(Deserialize, Debug)]
pub struct RatesResponse {
    pub success: bool,
    pub timestamp: i64,
    pub base: String,
    pub date: String,
    pub rates: HashMap<String, f64>,
}
