use crate::{database::schema::measurement, modules::conversion::models::core::{Unit, UnitSubType, UnitType}};
use crate::{
    database::connection::get_connection_diesel,
    modules::conversion::models::measurement::Measurement,
};
use diesel::prelude::*;

#[derive(Insertable, Default, serde::Deserialize)]
#[table_name = "measurement"]
pub struct NewMeasurement {
    pub rate: f64,
    pub is_base: bool,
    pub name: String,
    pub code: String,
    pub symbol: String,
    pub subtype: String,
}
impl NewMeasurement {
    pub fn to_unit(&self) -> Unit {
        Unit {
            name: String::from(self.name.as_str()),
            code: String::from(self.code.as_str()),
            symbol: String::from(self.symbol.as_str()),
            unit_type: UnitType::MEASUREMENT,
            subtype: None, // FIX THIS!
        }
    }
}


type MeasurementResult = Result<Measurement, &'static str>;
pub struct MeasurementRepository;
impl MeasurementRepository {
    pub fn create(m: Measurement) -> MeasurementResult {
        let connection = get_connection_diesel()?;

        let code = String::from(m.code.as_str());

        let new_measurement = NewMeasurement {
            name: m.name,
            rate: m.rate,
            is_base: m.is_base,
            symbol: m.symbol,
            code: m.code,
            subtype: m.subtype,
        };

        diesel::insert_into(measurement::table)
            .values(&new_measurement)
            .execute(&connection)
            .map_or(Err("Not able to create measurement."), |_| {
                MeasurementRepository::get(code.as_str())
            })
    }

    pub fn update(m: Measurement) -> MeasurementResult {
        let connection = get_connection_diesel()?;
        diesel::update(&m)
            .set(&m)
            .execute(&connection)
            .map_or(Err("Not able to update measurement."), |_| {
                MeasurementRepository::get(m.code.as_str())
            })
    }

    pub fn save(m: Measurement) -> MeasurementResult {
        return if m.id == 0 {
            MeasurementRepository::create(m)
        } else {
            MeasurementRepository::update(m)
        };
    }

    pub fn get(measurement_code: &str) -> MeasurementResult {
        use crate::database::schema::measurement::dsl::*;

        let connection = get_connection_diesel()?;
        measurement
            .filter(code.eq(measurement_code))
            .first::<Measurement>(&connection)
            .map_err(|_| "Measurement not found.")
    }

    pub fn get_all() -> Result<Vec<Measurement>, &'static str> {
        use crate::database::schema::measurement::dsl::*;

        let connection = get_connection_diesel()?;
        measurement
            .load::<Measurement>(&connection)
            .map_err(|_| "No currencies found.")
    }
}
