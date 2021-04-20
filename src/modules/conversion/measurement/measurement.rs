extern crate measurements;

use crate::modules::conversion::{
    data_controller::get_measurements,
    models::core::{Conversion, ConversionResult, Unit, UnitSubType},
    repository::measurement_repository::NewMeasurement,
};

use measurements::{Length, Measurement, Temperature};

use super::super::{
    measurement::utils::MeasurementUtils, models::measurement,
    repository::measurement_repository::MeasurementRepository,
};

// fn save_units<T>()
// where
//     T: MeasurementUtils,
//     T: Measurement,
// {
//     if let Ok(base_unit) = find_unit::<T>(T::get_base_code().as_str()) {
//         let base_value = 1.0;
//         if let Ok(base) = T::from_code(base_unit.code.as_str(), base_value) {
//             for unit in T::get_all_units() {
//                 if let Ok(converted) = base.to(unit.code.as_str()) {
//                     let value = converted;
//                     if let Some(subtype) = unit.subtype {
//                         let m = measurement::Measurement {
//                             id: 0,
//                             rate: value,
//                             is_base: base_unit.code == unit.code,
//                             name: unit.name,
//                             code: unit.code,
//                             symbol: unit.symbol,
//                             subtype: subtype.to_string(),
//                         };
//                         println!("{:?}", m);
//                         // MeasurementRepository::save(m);
//                     }
//                 }
//             }
//         }
//     };
// }

// pub fn save_all_units() {
//     save_units::<Length>();
//     save_units::<Temperature>();
// }

// pub fn get_units() -> Vec<Unit> {
//     let mut units = Vec::new();

//     for unit in Length::get_all_units() {
//         units.push(unit);
//     }
//     for unit in Temperature::get_all_units() {
//         units.push(unit);
//     }

//     units
// }

// pub fn find_unit<T>(text: &str) -> Result<Unit, &'static str>
// where
//     T: MeasurementUtils,
// {
//     for unit in T::get_all_units() {
//         if unit.code == text || unit.name == text || unit.symbol == text {
//             return Ok(unit);
//         }
//     }
//     Err("Unit not found.")
// }

// pub fn to_unit(text: &str) -> Result<Unit, &'static str> {
//     if let Ok(unit) = find_unit::<Length>(text) {
//         return Ok(unit);
//     }

//     if let Ok(unit) = find_unit::<Temperature>(text) {
//         return Ok(unit);
//     }

//     Err("Unit not found.")
// }

fn convert_to_measurements(
    from: &'static str,
    to: Vec<&'static str>,
) -> (Option<NewMeasurement>, Vec<NewMeasurement>) {
    let mut codes = Vec::new();
    codes.push(from);
    for code in to {
        codes.push(code);
    }

    let mut base: Option<NewMeasurement> = None;
    let mut measurements = Vec::new();

    for measurement in get_measurements(codes) {
        if measurement.code == from {
            base = Some(measurement);
        } else {
            measurements.push(measurement);
        }
    }

    (base, measurements)
}

pub async fn convert(
    from: &'static str,
    value: f64,
    to: Vec<&'static str>,
) -> Result<ConversionResult, &'static str> {
    let (base, measurements) = convert_to_measurements(from, to);
    let base = base.ok_or("Base not found.")?;

    let mut conversions = Vec::new();

    for measurement in measurements {
        let converted = if from == "f" && measurement.code == "c" {
            (value - 32.0) / 1.8
        } else if from == "c" && measurement.code == "f" {
            (value * 1.8) + 32.0
        } else {
            (measurement.rate * value) / base.rate
        };

        conversions.push(Conversion {
            unit: measurement.to_unit(),
            value: converted,
        })
    }

    let result = ConversionResult::new_with_to(
        Conversion {
            unit: base.to_unit(),
            value,
        },
        conversions,
    );

    Ok(result)
}
