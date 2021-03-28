use std::collections::HashMap;

use measurements::{Length, Measurement, Temperature};

use crate::modules::conversion::models::{Unit, UnitType};


type Units = HashMap<&'static str, (Unit, fn(f64) -> f64, fn(f64) -> f64)>;
pub fn get_units() -> Units {
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

pub fn to_unit(text: String) -> Result<Unit, &'static str> {
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