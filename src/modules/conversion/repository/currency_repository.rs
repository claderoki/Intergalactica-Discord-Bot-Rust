use crate::database::schema::currency;
use crate::{
    database::connection::get_connection_diesel, modules::conversion::models::currency::Currency,
};
use diesel::prelude::*;

#[derive(Insertable, Default)]
#[table_name = "currency"]
pub struct NewCurrency {
    pub rate: f64,
    pub is_base: bool,
    pub name: String,
    pub code: String,
    pub symbol: String,
}

// trait CRUDRepository {
//     fn create() -> Self {}
//     fn read() -> Self {}
//     fn update() -> Self {}
//     fn delete() -> Self {}
// }

type CurrencyResult = Result<Currency, &'static str>;
pub struct CurrencyRepository;
impl CurrencyRepository {
    pub fn create(c: Currency) -> CurrencyResult {
        let connection = get_connection_diesel()?;

        let code = String::from(c.code.as_str());

        let new_currency = NewCurrency {
            name: c.name,
            rate: c.rate,
            is_base: c.is_base,
            symbol: c.symbol,
            code: c.code,
        };

        diesel::insert_into(currency::table)
            .values(&new_currency)
            .execute(&connection)
            .map_or(Err("Not able to create currency."), |_| {
                CurrencyRepository::get(code.as_str())
            })
    }

    pub fn update(c: Currency) -> CurrencyResult {
        let connection = get_connection_diesel()?;
        diesel::update(&c)
            .set(&c)
            .execute(&connection)
            .map_or(Err("Not able to update currency."), |_| {
                CurrencyRepository::get(c.code.as_str())
            })
    }

    pub fn save(c: Currency) -> CurrencyResult {
        return if c.id == 0 {
            CurrencyRepository::create(c)
        } else {
            CurrencyRepository::update(c)
        };
    }

    pub fn get(currency_code: &str) -> CurrencyResult {
        use crate::database::schema::currency::dsl::*;

        let connection = get_connection_diesel()?;
        currency
            .filter(code.eq(currency_code))
            .first::<Currency>(&connection)
            .map_err(|_| "Currency not found.")
    }

    pub fn get_all() -> Result<Vec<Currency>, &'static str> {
        use crate::database::schema::currency::dsl::*;

        let connection = get_connection_diesel()?;
        currency
            .load::<Currency>(&connection)
            .map_err(|_| "No currencies found.")
    }

    pub fn get_multiple(currency_codes: Vec<&str>) -> Result<Vec<Currency>, &'static str> {
        use crate::database::schema::currency::dsl::*;

        let connection = get_connection_diesel()?;
        currency
            .load::<Currency>(&connection)
            .map_err(|_| "No currencies found.")
    }

    // fn update_rate(code: &'static str, rate: i64) -> CurrencyResult {
    //     let connection = get_connection_diesel()?;

    //     diesel::update(&h)
    //         .set(&h)
    //         .execute(&conn);
    //     Err("OK")
    //   }
}
