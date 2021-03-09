extern crate mysql;

use std::{collections::{HashMap}, fs::File, io::{BufReader}, env};
use super::{super::super::super::wrappers::fixerio::api::Fixerio, super::models::{Unit, UnitType, ConversionResult, Conversion}};

use mysql as my;

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
pub async fn get_symbols() -> Result<HashMap<String, String>, &'static str> {
    let fixerio = Fixerio::new(env::var("FIXERIO_ACCESS_KEY").expect("No fixerio access key set."));
    let symbols = fixerio.get_symbols().await;

    match symbols {
        Ok(data) => {
            return Ok(data.symbols);
        }
        Err(e) => {
            return Err(e)
        }
    }
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

pub async fn get_currency_symbol(code : String) -> Option<String> {
    match get_currencies() {
        Ok(currencies) => {
            if let Some(symbol) = currencies.get(code.as_str()) {   
                return Some(String::from(symbol.as_str()));
            } else {
                return None;
            }
        },
        Err(e) => {
            println!("Error getting currencies: {}", e);
            return None;
        }
    }
}

pub async fn get_currency_name(code : String) -> Option<String> {
    match get_symbols().await {
        Ok(symbols) => {
            if let Some(name) = symbols.get(code.as_str()) {
                return Some(String::from(name.as_str()));
            } else {
                return None;
            }
        },
        Err(e) => {
            return None;
        }
    }
}

pub async fn get_all_currencies() {
    let url = "mysql://root:password@localhost:3307/db_name";

    let pool = match my::Pool::new(url) {
        Ok(data) => {data},
        Err(_) => {return ();}
    };

    for mut stmt in pool.prepare(r"INSERT INTO payment
                                       (customer_id, amount, account_name)
                                   VALUES
                                       (:customer_id, :amount, :account_name)").into_iter() {
        for p in payments.iter() {
            // `execute` takes ownership of `params` so we pass account name by reference.
            // Unwrap each result just to make sure no errors happened.
            stmt.execute(params!{
                "customer_id" => p.customer_id,
                "amount" => p.amount,
                "account_name" => &p.account_name,
            }).unwrap();
        }
    }
}

pub async fn get_currency_unit(code : String) -> Unit {
    Unit::new_currency(
        String::from(code.as_str()),
        get_currency_name(String::from(code.as_str())).await,
        get_currency_symbol(String::from(code.as_str())).await
    )
}

pub async fn convert(from : &'static str, value : f64, to : Vec<&'static str>) -> Result<ConversionResult, &'static str> {
    let rates = get_rates().await?;

    let mut result = ConversionResult::new(Conversion {
        unit  : get_currency_unit(String::from(from)).await,
        value : value
    });

    let base_value : f64 = rates[&String::from(from)];
    for currency in to {
        result.to.push(Conversion {
            unit  : Unit::new_currency(String::from(currency), None, None),
            value : (rates[&String::from(currency)] * value) / base_value
        });
    }

    Ok(result)
}
