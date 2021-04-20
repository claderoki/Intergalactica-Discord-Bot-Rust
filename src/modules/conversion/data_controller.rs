use std::{collections::HashMap, fs::File, io::BufReader};

use once_cell::sync::Lazy;

use super::{
    models::{currency::Currency},
    repository::{currency_repository::CurrencyRepository, measurement_repository::NewMeasurement},
};

pub static CONFIG: Lazy<HashMap<String, bool>> = Lazy::new(|| {
        let m = HashMap::new();
        m.insert("all_loaded".into(), false);
        m
    }
);

pub static CURRENCIES: Lazy<std::sync::Mutex<HashMap<String, Currency>>> =
    Lazy::new(|| std::sync::Mutex::new(HashMap::new()));
pub static MEASUREMENTS: Lazy<std::sync::Mutex<HashMap<String, NewMeasurement>>> =
    Lazy::new(|| std::sync::Mutex::new(HashMap::new()));

fn load_all() {
    if let Some(mut all_loaded) = CONFIG.get("all_loaded") {
        println!("LOADING");
        load_currencies();
        load_measurements();
        all_loaded = &true;
    }
}

fn load_currencies() {
    if let Ok(mut currency_mapping) = CURRENCIES.lock() {
        if let Ok(currencies) = CurrencyRepository::get_all() {
            for currency in currencies {
                currency_mapping.insert(String::from(currency.code.as_str()), currency);
            }
        }
    }
}

fn get_data_measurements() -> Result<Vec<NewMeasurement>, &'static str> {
    if let Ok(file) = File::open("src/modules/conversion/data/measurements.json") {
        let reader = BufReader::new(file);
        let currencies: Result<Vec<NewMeasurement>, serde_json::Error> = serde_json::from_reader(reader);
        match currencies {
            Err(_) => {}
            Ok(d) => {
                return Ok(d);
            }
        }
    };

    Err("")
}

fn load_measurements() {
    if let Ok(mut measurement_mapping) = MEASUREMENTS.lock() {
        if let Ok(measurements) = get_data_measurements() {
            for measurement in measurements {
                measurement_mapping.insert(String::from(measurement.code.as_str()), measurement);
            }
        }
    }
}

pub fn get_measurements(codes: Vec<&'static str>) -> Vec<NewMeasurement> {
    load_all();
    let mut measurements: Vec<NewMeasurement> = Vec::new();

    if let Ok(measurement_mapping) = MEASUREMENTS.lock() {
        for code in codes {
            if let Some(measurement) = measurement_mapping.get(code) {
                measurements.push(*measurement);
            }
        }
    }
    measurements
}

pub fn get_currencies(codes: Vec<&'static str>) -> Vec<Currency> {
    load_all();
    let mut currencies: Vec<Currency> = Vec::new();

    if let Ok(currency_mapping) = CURRENCIES.lock() {
        for code in codes {
            if let Some(currency) = currency_mapping.get(code) {
                currencies.push(*currency);
            }
        }
    }
    currencies
}
