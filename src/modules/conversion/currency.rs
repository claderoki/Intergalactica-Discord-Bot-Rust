
extern crate reqwest;
extern crate serde;

use std::{collections::HashMap, error};

use serde::Deserialize;
use reqwest::Error;

enum FixerioEndpoint {
    SYMBOLS,
    LATEST,
}

impl FixerioEndpoint {
    pub fn get_name(&self) -> String {
        match self {
            Self::SYMBOLS => String::from("symbols"),
            Self::LATEST => String::from("latest"),
        }
    }
}

#[derive(Debug)]
pub struct Symbol {
    code: String,
    name: String
}
#[derive(Deserialize, Debug)]
pub struct SymbolsResponse {
    success : bool,
    symbols : HashMap<String, String>
}

#[derive(Deserialize, Debug)]
pub struct RatesResponse {
    success: bool,
    timestamp: i64,
    base: String,
    date: String,
    rates: HashMap<String, f64>,
}

pub struct Fixerio {
    access_key : String,
    base_url   : String
}

impl Fixerio {
    pub fn new(access_key : String) -> Self {
        Self {
            access_key: access_key,
            base_url: String::from("http://data.fixer.io/api"),
        }
    }

    fn get_base_uri(&self, endpoint : FixerioEndpoint) -> String {
        let mut uri = String::from(self.base_url.as_str());
        uri.push_str("/");
        uri.push_str(endpoint.get_name().as_str());
        uri.push_str("?");
        uri.push_str("access_key=");
        uri.push_str(self.access_key.as_str());
        uri
    }

    pub async fn get_symbols(&self) -> Result<Vec<Symbol>, &'static str> {
        let uri = self.get_base_uri(FixerioEndpoint::SYMBOLS);

        let response = match reqwest::get(uri.as_str()).await {
            Ok(response) => {
                response
            },
            Err(e) => {
                println!("{:?}", e);
                return Err("something went wrong");
            }
        };

        let data = match response.json::<SymbolsResponse>().await {
            Ok(data) => {
                data
            },
            Err(e) => {
                println!("{:?}", e);
                return Err("something went wrong");
            }
        };

        let mut symbols = Vec::new();

        for (key, value) in data.symbols.iter() {
            symbols.push(Symbol { code : String::from(key.as_str()), name : String::from(value)});
        }

        Ok(symbols)
    }

    pub async fn get_rates(&self) -> Result<RatesResponse, &'static str> {
        let uri = self.get_base_uri(FixerioEndpoint::LATEST);

        let response = match reqwest::get(uri.as_str()).await {
            Ok(response) => {
                response
            },
            Err(e) => {
                println!("{:?}", e);
                return Err("something went wrong");
            }
        };

        let data = match response.json::<RatesResponse>().await {
            Ok(data) => {
                data
            },
            Err(e) => {
                println!("{:?}", e);
                return Err("something went wrong");
            }
        };

        Ok(data)
    }

}
