extern crate measurements;

use std::collections::HashMap;

use measurements::{Length, Measurement, Temperature};

use super::models::{Conversion, ConversionResult, Unit, UnitType};

fn to_unit(text: String) -> Result<Unit, &'static str> {
    let result = match text.to_lowercase().as_str() {
        "c" | "celsius" => Ok(Unit {
            name: "celsius",
            code: "c",
            symbol: "°C",
            unit_type: UnitType::MEASUREMENT,
        }),

        "f" | "fahrenheit" => Ok(Unit {
            name: "fahrenheit",
            code: "f",
            symbol: "°F",
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
            Unit::new("celsius", "c", "°C", UnitType::MEASUREMENT),
            |x: f64| Temperature::from_celsius(x).as_base_units(),
            |x: f64| Temperature::from_base_units(x).as_celsius(),
        ),
    );
    hm.insert(
        "f",
        (
            Unit::new("fahrenheit", "f", "°F", UnitType::MEASUREMENT),
            |x: f64| Temperature::from_fahrenheit(x).as_base_units(),
            |x: f64| Temperature::from_base_units(x).as_fahrenheit(),
        ),
    );
    hm.insert(
        "m",
        (
            Unit::new("meters", "m", "m", UnitType::MEASUREMENT),
            |x: f64| Length::from_meters(x).as_base_units(),
            |x: f64| Length::from_base_units(x).as_meters(),
        ),
    );
    hm.insert(
        "ft",
        (
            Unit::new("feet", "ft", "\"", UnitType::MEASUREMENT),
            |x: f64| Length::from_feet(x).as_base_units(),
            |x: f64| Length::from_base_units(x).as_feet(),
        ),
    );

    hm
}

impl Conversion {
    fn convert(&self) -> Result<ConversionResult, &'static str> {
        let units = get_units();
        let (_, to_base, _) = units.get(self.unit.code).ok_or("unit no on the list")?;
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

pub fn convert_measurement(value: f64, from: String) -> Result<ConversionResult, &'static str> {
    let unit = to_unit(from)?;
    let conv = Conversion { unit, value };
    conv.convert()
}
