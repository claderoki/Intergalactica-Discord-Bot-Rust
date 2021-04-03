use measurements::{Length, Temperature};

use crate::modules::conversion::models::{Conversion, ConversionResult, Unit, UnitType};

trait MeasurementUtils {
    fn from_code(code: &'static str, value: f64) -> Result<Self, &'static str>
    where
        Self: Sized;
    fn to(&self, code: &'static str) -> Result<f64, &'static str>;
    fn get_all_units() -> Vec<Unit>;
}

impl MeasurementUtils for Temperature {
    fn from_code(code: &'static str, value: f64) -> Result<Self, &'static str> {
        Ok(match code {
            "c" => Temperature::from_celsius(value),
            "f" => Temperature::from_fahrenheit(value),
            "k" => Temperature::from_kelvin(value),
            _ => {
                return Err("Not found.");
            }
        })
    }

    fn to(&self, code: &'static str) -> Result<f64, &'static str> {
        Ok(match code {
            "c" => self.as_celsius(),
            "f" => self.as_fahrenheit(),
            "k" => self.as_kelvin(),
            _ => return Err("Not found."),
        })
    }

    fn get_all_units() -> Vec<Unit> {
        let mut units = Vec::new();
        units.push(Unit::new(
            "celsius".to_string(),
            "c".to_string(),
            "°C".to_string(),
            UnitType::MEASUREMENT,
        ));
        units.push(Unit::new(
            "fahrenheit".to_string(),
            "f".to_string(),
            "°F".to_string(),
            UnitType::MEASUREMENT,
        ));
        units.push(Unit::new(
            "kelvin".to_string(),
            "k".to_string(),
            "K".to_string(),
            UnitType::MEASUREMENT,
        ));

        units
    }
}

impl MeasurementUtils for Length {
    fn from_code(code: &'static str, value: f64) -> Result<Self, &'static str> {
        Ok(match code {
            "m" => Length::from_meters(value),
            "mm" => Length::from_millimeters(value),
            "cm" => Length::from_centimeters(value),
            "hm" => Length::from_hectometers(value),
            "km" => Length::from_kilometers(value),
            "inch" => Length::from_inches(value),
            "ft" => Length::from_feet(value),
            "yd" => Length::from_yards(value),
            "ml" => Length::from_miles(value),
            _ => return Err("Not found."),
        })
    }

    fn to(&self, code: &'static str) -> Result<f64, &'static str> {
        Ok(match code {
            "m" => self.as_meters(),
            "mm" => self.as_millimeters(),
            "cm" => self.as_centimeters(),
            "hm" => self.as_hectometers(),
            "km" => self.as_kilometers(),
            "inch" => self.as_inches(),
            "ft" => self.as_feet(),
            "yd" => self.as_yards(),
            "ml" => self.as_miles(),
            _ => return Err("Not found."),
        })
    }

    fn get_all_units() -> Vec<Unit> {
        let mut units = Vec::new();

        units.push(Unit::new(
            "meters".to_string(),
            "m".to_string(),
            "m".to_string(),
            UnitType::MEASUREMENT,
        ));
        units.push(Unit::new(
            "millimeters".to_string(),
            "mm".to_string(),
            "mm".to_string(),
            UnitType::MEASUREMENT,
        ));
        units.push(Unit::new(
            "centimeters".to_string(),
            "cm".to_string(),
            "cm".to_string(),
            UnitType::MEASUREMENT,
        ));
        units.push(Unit::new(
            "kilometers".to_string(),
            "km".to_string(),
            "km".to_string(),
            UnitType::MEASUREMENT,
        ));
        units.push(Unit::new(
            "inches".to_string(),
            "inch".to_string(),
            "\"".to_string(),
            UnitType::MEASUREMENT,
        ));
        units.push(Unit::new(
            "feet".to_string(),
            "ft".to_string(),
            "'".to_string(),
            UnitType::MEASUREMENT,
        ));
        units.push(Unit::new(
            "yards".to_string(),
            "yd".to_string(),
            "".to_string(),
            UnitType::MEASUREMENT,
        ));
        units.push(Unit::new(
            "miles".to_string(),
            "mi".to_string(),
            "mi".to_string(),
            UnitType::MEASUREMENT,
        ));

        units
    }
}

pub fn get_units() -> Vec<Unit> {
    let mut units = Vec::new();

    for unit in Length::get_all_units() {
        units.push(unit);
    }
    for unit in Temperature::get_all_units() {
        units.push(unit);
    }

    units
}

pub fn to_unit(text: &'static str) -> Result<Unit, &'static str> {
    // Pretty inefficient. Maybe cache the get_units somewhere?
    for unit in get_units() {
        if unit.code == text || unit.name == text || unit.symbol == text {
            return Ok(unit);
        }
    }

    Err("Unit not found.")
}
pub async fn convert(
    from: &'static str,
    value: f64,
    to: Vec<&'static str>,
) -> Result<ConversionResult, &'static str> {
    let base_unit = to_unit(from)?;
    // Add a way to find out what kind of unit a code is from?
    let base = Temperature::from_code(from, 50.0)?;

    let mut result = ConversionResult::new(Conversion {
        unit: base_unit,
        value,
    });

    for code in to {
        if let Ok(unit) = to_unit(code) {
            if let Ok(value) = base.to(code) {
                result.to.push(Conversion { unit, value })
            }
        }
    }

    Ok(result)
}
