extern crate measurements;

use measurements::Temperature;

use super::models::{Conversion, ConversionResult, Unit, UnitType};

fn to_unit(text: String) -> Result<Unit, &'static str> {
    let result = match text.to_lowercase().as_str() {
        "c" | "celsius" => Ok(Unit::celsius()),
        "f" | "fahrenheit" => Ok(Unit::fahrenheit()),
        _ => Err("No units found"),
    };
    result
}

pub fn convert_measurement(value: f64, from: String) -> Result<ConversionResult, &'static str> {
    let unit = to_unit(from);

    let base = match unit {
        Ok(u) => Some(Conversion {
            unit: u,
            value: value,
        }),
        Err(e) => return Err(e),
    };

    // check if we can pass here without unwrapping
    let mut result = ConversionResult::new(base.unwrap());
    match result.base.unit.code.as_str() {
        "f" => {
            let temperature = Temperature::from_fahrenheit(value);
            result.to.push(Conversion {
                unit: Unit::celsius(),
                value: temperature.as_celsius(),
            })
        }
        "c" => {
            let temperature = Temperature::from_celsius(value);
            result.to.push(Conversion {
                unit: Unit::fahrenheit(),
                value: temperature.as_fahrenheit(),
            })
        }
        _ => {
            panic!("something");
        }
    }

    return Ok(result);
}