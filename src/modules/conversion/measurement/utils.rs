use crate::modules::conversion::models::{Unit, UnitSubType};
use measurements::{Length, Temperature};

pub trait MeasurementUtils {
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
        let subtype = UnitSubType::TEMPERATURE;
        vec![
            Unit::new_measurement("celsius", "c", "°C", subtype),
            Unit::new_measurement("fahrenheit", "f", "°F", subtype),
            Unit::new_measurement("kelvin", "k", "K", subtype),
        ]
    }
}

impl MeasurementUtils for Length {
    fn from_code(code: &'static str, value: f64) -> Result<Self, &'static str> {
        Ok(match code {
            "m" => Length::from_meters(value),
            "mm" => Length::from_millimeters(value),
            "cm" => Length::from_centimeters(value),
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
            "km" => self.as_kilometers(),
            "inch" => self.as_inches(),
            "ft" => self.as_feet(),
            "yd" => self.as_yards(),
            "ml" => self.as_miles(),
            _ => return Err("Not found."),
        })
    }

    fn get_all_units() -> Vec<Unit> {
        let subtype = UnitSubType::LENGTH;

        vec![
            Unit::new_measurement("meters", "m", "m", subtype),
            Unit::new_measurement("millimeters", "mm", "mm", subtype),
            Unit::new_measurement("centimeters", "cm", "cm", subtype),
            Unit::new_measurement("kilometers", "km", "km", subtype),
            Unit::new_measurement("inches", "inch", "\"", subtype),
            Unit::new_measurement("feet", "ft", "'", subtype),
            Unit::new_measurement("yards", "yd", "", subtype),
            Unit::new_measurement("miles", "mi", "mi", subtype),
        ]
    }
}
