/*
https://github.com/jocull/rust-measurements

Conversions:
Measurements:
- When user types a message, example: 35f it will send an embed with the other measurements (celcius)

Currency:
When user types either €50, 50€ or 50eur it will show the conversions of the currencies
of all the users that typed in the last 20 messages


class Conversion:
    pass
class CurrencyConversion(Conversion):
    pass
class TimezoneConversion(Conversion):
    pass
class MeasurementConversion(Conversion):
    pass

measurements = [
    ("c", "f"),
    ("kg", "lb"),
    ("g", "oz"),
    ("cm", "inch", "ft"),
    ("ml", "us_cup"),
    ("km", "mi"),
    ("m", "yd", "ft"),
]

name = "Fahrenheit", code = "f", symbol = °F
*/

extern crate measurements;
use measurements::Temperature;

#[derive(Debug)]
pub struct Unit {
    pub name: String,
    pub code: String,
    pub symbol: String,
}

impl Unit {
    pub fn celsius() -> Unit {
        Unit {
            name: String::from("celsius"),
            code: String::from("c"),
            symbol: String::from("°C"),
        }
    }

    pub fn fahrenheit() -> Unit {
        Unit {
            name: String::from("fahrenheit"),
            code: String::from("f"),
            symbol: String::from("°F"),
        }
    }
}

#[derive(Debug)]
pub struct Conversion {
    pub unit: Unit,
    pub value: f64,
}

impl Conversion {
    pub fn to_string(&self) -> String {
        format!("{}{}", self.value, self.unit.symbol)
    }
}

pub struct ConversionResult {
    pub base: Conversion,
    pub to: Vec<Conversion>,
}

impl ConversionResult {
    pub fn new(base: Conversion) -> ConversionResult {
        ConversionResult {
            base: base,
            to: Vec::new(),
        }
    }
}

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

    // let mut measurement_mapping = Vec::new();
    // measurement_mapping.push(vec!["f", "c", "k"]);
    // measurement_mapping.push(vec!["kg", "lb"]);
    // measurement_mapping.push(vec!["cm", "inch", "ft"]);
    // measurement_mapping.push(vec!["g", "oz"]);
    // measurement_mapping.push(vec!["ml", "us_cup"]);
    // measurement_mapping.push(vec!["km", "mi"]);
    // measurement_mapping.push(vec!["m", "yd", "ft"]);

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

// fn main() {
//     // Lengths!
//     let football_field = Length::from_yards(100.0);
//     let meters = football_field.as_meters();
//     println!("There are {} meters in a football field.", meters);

//     /// Temperatures!
//     let boiling_water = Temperature::from_celsius(100.0);
//     let fahrenheit = boiling_water.as_fahrenheit();
//     println!("Boiling water measures at {} degrees fahrenheit.", fahrenheit);

//     // Weights!
//     let metric_ton = Weight::from_metric_tons(1.0);
//     let united_states_tons = metric_ton.as_short_tons();
//     let united_states_pounds = metric_ton.as_pounds();
//     println!("One metric ton is {} U.S. tons - that's {} pounds!", united_states_tons, united_states_pounds);

//     // Volumes!
//     let gallon = Volume::from_gallons(1.0);
//     let pint = Volume::from_pints(1.0);
//     let beers = gallon / pint;
//     println!("A gallon of beer will pour {:.1} pints!", beers);

//     // Pressures!
//     let atmosphere = Pressure::from_atmospheres(1.0);
//     println!("Earth's atmosphere is usually {} psi", atmosphere.as_psi());
// }
