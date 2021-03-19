extern crate measurements;

use std::collections::HashMap;

use measurements::{Length, Measurement, Temperature};

use super::{currency::currency::get_all_currencies, models::{Conversion, ConversionResult, Unit, UnitType}};

fn to_unit(text: String) -> Result<Unit, &'static str> {
    let result = match text.to_lowercase().as_str() {
        "c" | "celsius" => Ok(Unit {
            name: "celsius".to_string(),
            code: "c".to_string(),
            symbol: "°C".to_string(),
            unit_type: UnitType::MEASUREMENT,
        }),

        "f" | "fahrenheit" => Ok(Unit {
            name: "fahrenheit".to_string(),
            code: "f".to_string(),
            symbol: "°F".to_string(),
            unit_type: UnitType::MEASUREMENT,
        }),
        _ => Err("No units found"),
    };
    result
}

type Units = HashMap<&'static str, (Unit, fn(f64) -> f64, fn(f64) -> f64)>;
fn get_units() -> Units {
    let mut hm: Units = HashMap::new();

    hm.insert(
        "c",
        (
            Unit::new(
                "celsius".to_string(),
                "c".to_string(),
                "°C".to_string(),
                UnitType::MEASUREMENT,
            ),
            |x: f64| Temperature::from_celsius(x).as_base_units(),
            |x: f64| Temperature::from_base_units(x).as_celsius(),
        ),
    );
    hm.insert(
        "f",
        (
            Unit::new(
                "fahrenheit".to_string(),
                "f".to_string(),
                "°F".to_string(),
                UnitType::MEASUREMENT,
            ),
            |x: f64| Temperature::from_fahrenheit(x).as_base_units(),
            |x: f64| Temperature::from_base_units(x).as_fahrenheit(),
        ),
    );
    hm.insert(
        "m",
        (
            Unit::new(
                "meters".to_string(),
                "m".to_string(),
                "m".to_string(),
                UnitType::MEASUREMENT,
            ),
            |x: f64| Length::from_meters(x).as_base_units(),
            |x: f64| Length::from_base_units(x).as_meters(),
        ),
    );
    hm.insert(
        "ft",
        (
            Unit::new(
                "feet".to_string(),
                "ft".to_string(),
                "\"".to_string(),
                UnitType::MEASUREMENT,
            ),
            |x: f64| Length::from_feet(x).as_base_units(),
            |x: f64| Length::from_base_units(x).as_feet(),
        ),
    );

    hm
}

impl Conversion {
    fn convert(&self) -> Result<ConversionResult, &'static str> {
        let units = get_units();
        let (_, to_base, _) = units
            .get(self.unit.code.as_str())
            .ok_or("unit no on the list")?;
        let base = to_base(self.value);

        let conversions =
            units
                .iter()
                .filter(|(&k, _)| &k != &self.unit.code)
                .map(|(_, (u, _, from_base))| Conversion {
                    unit: u.clone(),
                    value: from_base(base),
                });

        Ok(ConversionResult {
            base: self.clone(),
            to: conversions.collect(),
        })
    }
}

pub fn get_all_codes_and_symbols() -> Vec<String> {
    let mut values = Vec::new();

    for (code, (unit, _, _)) in get_units() {
        values.push(String::from(code).to_lowercase());
        values.push(unit.symbol.to_lowercase());
    }

    for currency in get_all_currencies() {
        values.push(currency.code.to_lowercase());
        values.push(currency.symbol.to_lowercase());
    }

    return values;
}

pub fn convert_measurement(value: f64, from: String) -> Result<ConversionResult, &'static str> {
    let unit = to_unit(from)?;
    let conv = Conversion { unit, value };
    conv.convert()
}
