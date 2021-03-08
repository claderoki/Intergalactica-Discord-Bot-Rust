use std::{collections::HashMap, env};

use super::{super::super::wrappers::fixerio::api::Fixerio, models::Conversion, models::ConversionResult, models::{Unit, UnitType}};

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

pub async fn convert(from : &'static str, value : f64, to : Vec<&'static str>) -> Result<ConversionResult, &'static str> {
    let rates = get_rates().await?;

    let mut result = ConversionResult::new(Conversion {
        unit  : Unit::new_currency(String::from(from), None, None),
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
