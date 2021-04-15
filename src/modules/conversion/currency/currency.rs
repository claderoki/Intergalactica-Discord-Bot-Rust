extern crate mysql;

use std::{collections::HashMap, env};

use crate::{
    modules::conversion::{
        models::{
            core::{Conversion, ConversionResult, Unit, UnitType},
            currency::Currency,
        },
        repository::currency_repository::CurrencyRepository,
    },
    wrappers::fixerio::api::Fixerio,
};

/*TODO: cache*/
pub async fn get_rates() -> Result<HashMap<String, f64>, &'static str> {
    let fixerio = Fixerio::new(env::var("FIXERIO_ACCESS_KEY").expect("No fixerio access key set."));
    let rates = fixerio.get_rates().await;

    match rates {
        Ok(data) => {
            return Ok(data.rates);
        }
        Err(e) => return Err(e),
    }
}

pub async fn update_currencies() -> Result<(), &'static str> {
    let db_currencies = CurrencyRepository::get_all()?;
    let api_rates = get_rates().await?;

    for mut currency in db_currencies {
        let code = currency.code.as_str();
        let rate = api_rates.get(code).ok_or("err")?;
        if currency.rate != *rate {
            currency.rate = *rate;
            CurrencyRepository::save(currency);
        }
    }

    return Ok(());
}

pub fn currency_to_unit(currency: &Currency) -> Unit {
    Unit {
        name: String::from(currency.name.as_str()),
        code: String::from(currency.code.as_str()),
        symbol: String::from(currency.symbol.as_str()),
        unit_type: UnitType::CURRENCY,
        subtype: None,
    }
}

pub async fn get_context_currency_codes() -> Vec<String> {
    vec![
        String::from("USD"),
        String::from("EUR"),
        String::from("PHP"),
    ]
}

pub async fn convert(
    from: &'static str,
    value: f64,
    to: Vec<&'static str>,
) -> Result<ConversionResult, &'static str> {
    let mut currencies = HashMap::new();

    // TODO: Add a parameter to get_all_currencies so we can only get the currencies we need.
    for currency in CurrencyRepository::get_all()? {
        currencies.insert(String::from(currency.code.as_str()), currency);
    }

    let base_currency = currencies.get(from).ok_or("Base not found.")?;

    let mut result = ConversionResult::new(Conversion {
        unit: currency_to_unit(base_currency),
        value,
    });

    for code in to {
        if let Some(currency) = currencies.get(code) {
            result.to.push(Conversion {
                unit: currency_to_unit(currency),
                value: (currency.rate * value) / base_currency.rate,
            });
        }
    }

    Ok(result)
}
