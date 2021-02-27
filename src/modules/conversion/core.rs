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

struct Unit {
    name: String,
    code: String,
    symbol: String,
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

struct Conversion {
    unit: Unit,
    value: f64,
}

struct ConversionResult {
    base: Conversion,
    to: Vec<Conversion>,
}

impl ConversionResult {
    pub fn new(base: Conversion) -> ConversionResult {
        ConversionResult {
            base: base,
            to: Vec::new(),
        }
    }
}

extern crate measurements;

use measurements::Temperature;

fn to_unit(text: String) -> Result<Unit, &'static str> {
    let name: String;
    let code: String;
    let symbol: String;

    let unit: Option<Unit> = match text.to_lowercase().as_str() {
        "c" | "celsius" => Some(Unit::celsius()),
        "f" | "fahrenheit" => Some(Unit::fahrenheit()),
        _ => None,
    };

    if let Some(u) = unit {
        Ok(u)
    } else {
        Err("No units found")
    }
}

pub fn convert_measurement(value: f64, from: String) -> Result<ConversionResult, &'static str> {
    let unit: Result<Unit, &'static str> = to_unit(from);

    let conversion = match unit {
        Ok(u) => Some(Conversion {
            unit: u,
            value: value,
        }),
        Err(e) => None,
    };

    let r = match Some(conversion) {
        Conversion => Some(ConversionResult::new(conversion.unwrap())),
        None => None,
    };

    if let Some(result) = r {
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
        }
        return Ok(result);
    } else {
        Err("");
    }
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
