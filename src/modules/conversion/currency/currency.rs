extern crate mysql;

use std::{collections::{HashMap}, fs::File, io::{BufReader}, env};
use mysql::{Conn, OptsBuilder, params, Row, prelude::Queryable, };
use once_cell::sync::Lazy;

use crate::SYMBOLS;

use super::{super::super::super::wrappers::fixerio::api::Fixerio, super::models::{Unit, UnitType, ConversionResult, Conversion}};
use super::models::currency::{Currency};

/*TODO: cache*/
pub async fn get_rates() -> Result<HashMap<String, f64>, &'static str> {
    let fixerio = Fixerio::new(env::var("FIXERIO_ACCESS_KEY").expect("No fixerio access key set."));
    let rates = fixerio.get_rates().await;

    match rates {
        Ok(data) => {
            return Ok(data.rates);
        }
        Err(e) => {
            return Err(e)
        }
    }
}

/*TODO: cache*/
pub async fn get_symbols() -> &'static Lazy<std::sync::Mutex<HashMap<String, String>>>  {
    let fixerio = Fixerio::new(env::var("FIXERIO_ACCESS_KEY").expect("No fixerio access key set."));
    let symbols = fixerio.get_symbols().await;

    if let Ok(sym) = symbols{
        SYMBOLS.lock().unwrap().clear();
        for (k, v) in sym.symbols {
            SYMBOLS.lock().unwrap().insert(k, v);
        }
    }

    &SYMBOLS
}

/*TODO: cache*/
pub fn get_currencies() -> Result<HashMap<String, String>, &'static str> {
    let file = match File::open("src/modules/conversion/currency/data/currencies.json") {
        Err(e) => {
            return Err("Error opening currencies file");
        }
        Ok(f) => f,
    };
    let reader = BufReader::new(file);
    let currencies : Result<HashMap<String, String>, serde_json::Error> = serde_json::from_reader(reader);
    match currencies {
        Err(e) => {
            return Err("Error converting JSON to struct");
        },
        Ok(d) => {
            return Ok(d);
        }
    }
}

pub async fn get_currency_symbol(code : &str) -> Option<String> {
    get_currencies().ok()?.get(code).cloned()
}

pub async fn get_currency_name(code : &str) -> Option<String> {
    get_symbols().await.lock().ok()?.get(code).cloned()
}

fn get_db_opts() -> OptsBuilder {
    OptsBuilder::new()
    .user(Some(env::var("DB_USER").expect("Expected DB_USER in the environment")))
    .db_name(Some(env::var("DB_NAME").expect("Expected DB_NAME in the environment")))
    .ip_or_hostname(Some(env::var("DB_HOST").expect("Expected DB_HOST in the environment")))
    .pass(Some(env::var("DB_PASSWORD").expect("Expected DB_PASSWORD in the environment")))
}

pub fn get_select_rows(query : &'static str) -> Vec<Row> {
    let mut rows : Vec<Row> = Vec::new();

    match Conn::new(get_db_opts()) {
        Ok(mut conn) => {
            if let Ok(mut result) = conn.query_iter(query) {
                while let Some(result_set) = result.next_set() {
                    if let Ok(set) = result_set {
                        for r in set {
                            if let Ok(row) = r {
                                rows.push(row);
                            }
                        }
                    }
                }
            }
        },
        Err(_) => {}
    }

    rows
}

pub fn get_all_currencies() -> Vec<Currency> {
    let mut currencies : Vec<Currency> = Vec::new();
    for row in get_select_rows("SELECT * FROM currency") {
        let currency = Currency::from_row(row);
        currencies.push(currency);
    }
    currencies
}

pub enum UpdateType {
    All,
    Rate,
}

pub async fn update_currencies(update_type : UpdateType) -> Result<(), &'static str> {
    let db_currencies = get_all_currencies();
    let api_rates     = get_rates().await?;

    let mut missing : Vec<String> = Vec::new();
    let mut currencies: Vec<Currency> = Vec::new();

    for (code, _) in &api_rates {
        let mut found = false;
        for currency in &db_currencies {
            if &currency.code == code {
                found = true;
            }
        }
        if !found {
            missing.push(String::from(code.as_str()));
        }
    }

    if missing.len() > 0 {
        let symbols = get_currencies()?;
        let names = get_symbols().await;

        if names.lock().is_ok() {
            for code in missing {
                let name = names.lock().unwrap().get(code.as_str()).cloned();
                let symbol = symbols.get(code.as_str());
                let rate = api_rates.get(code.as_str());
                if let (Some(name), Some(symbol), Some(rate)) = (name, symbol, rate) {
                    currencies.push(Currency {
                        id: 0,
                        rate: *rate,
                        is_base: code.as_str() == "EUR",
                        name: String::from(name.as_str()),
                        code: String::from(code.as_str()),
                        symbol: String::from(symbol.as_str())
                    });
                }
            }
        }
    }

    for mut currency in db_currencies {
        let code = currency.code.as_str();
        let rate = api_rates.get(code).ok_or("err")?;
        currency.rate = *rate;
        currencies.push(currency);
    }

    for currency in currencies {
        save_currency(currency);
    }

    return Ok(());
}

pub fn save_currency(currency : Currency) {
    let mut query = String::from("");
    if currency.id == 0 {
        query.push_str("INSERT INTO currency ");
        query.push_str("(`rate`, `is_base`, `name`, `code`, `symbol`)");
        query.push_str("VALUES ");
        query.push_str("(:rate, :is_base, :name, :code, :symbol)");
        println!("INSERT: {:?}", currency);
    } else {
        query.push_str("UPDATE currency ");
        query.push_str("SET `rate` = :rate, `is_base` = :is_base, `name` = :name, `code` = :code, `symbol` = :symbol ");
        query.push_str("WHERE `id` = :id");
        println!("UPDATE: {:?}", currency);
    }

    match Conn::new(get_db_opts()) {
        Ok(mut conn) => {
            let result = conn.exec::<i64,_,_>(
                query,
                params! {
                    "id" => currency.id,
                    "rate" => currency.rate,
                    "is_base" => currency.is_base,
                    "name" => currency.name,
                    "code" => currency.code,
                    "symbol" => currency.symbol
                }
            );
            println!("result: {:?}", result);
        },
        Err(e) => {
            println!("Error : {:?}", e);
        }
    }
}

pub fn currency_to_unit(currency : &Currency) -> Unit {
    Unit {
        name: String::from(currency.name.as_str()),
        code: String::from(currency.code.as_str()),
        symbol: String::from(currency.symbol.as_str()),
        unit_type: UnitType::CURRENCY
    }
}

pub async fn convert(from : &'static str, value : f64, to : Vec<&'static str>) -> Result<ConversionResult, &'static str> {
    let mut currencies = HashMap::new();

    // TODO: Add a parameter to get_all_currencies so we can only get the currencies we need.
    for currency in get_all_currencies() {
        currencies.insert(String::from(currency.code.as_str()), currency);
    }

    let base_currency = currencies.get(from).ok_or("Base not found.")?;

    let mut result = ConversionResult::new(Conversion {
        unit  : currency_to_unit(base_currency),
        value
    });

    for code in to {
        if let Some(currency) = currencies.get(code) {
            result.to.push(Conversion {
                unit  : currency_to_unit(currency),
                value : (currency.rate * value) / base_currency.rate
            });
        }
    }

    Ok(result)
}