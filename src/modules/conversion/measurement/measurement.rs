use crate::modules::conversion::models::{Conversion, ConversionResult, Unit, UnitSubType};
use measurements::{Length, Temperature};

use super::utils::MeasurementUtils;

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

fn get_conversions<T>(
    from: &'static str,
    value: f64,
    to: Vec<&'static str>,
) -> Result<Vec<Conversion>, &'static str>
where
    T: MeasurementUtils,
{
    let mut conversions = Vec::new();

    let base = T::from_code(from, value)?;

    for code in to {
        if let Ok(unit) = to_unit(code) {
            if let Ok(value) = base.to(code) {
                conversions.push(Conversion { unit, value })
            }
        }
    }

    Ok(conversions)
}

pub async fn convert(
    from: &'static str,
    value: f64,
    to: Vec<&'static str>,
) -> Result<ConversionResult, &'static str> {
    let mut base_unit = to_unit(from)?;

    let subtype = base_unit.subtype.ok_or("No subtype set.")?;

    let conversions = match subtype {
        UnitSubType::LENGTH => get_conversions::<Length>(from, value, to)?,
        UnitSubType::TEMPERATURE => get_conversions::<Temperature>(from, value, to)?,
    };

    base_unit.subtype = Some(subtype);

    let mut result = ConversionResult::new(Conversion {
        unit: base_unit,
        value,
    });
    result.to = conversions;

    Ok(result)
}
