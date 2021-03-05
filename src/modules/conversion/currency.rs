use std::{collections::HashMap, env};

use super::super::super::wrappers::fixerio::api::Fixerio;

pub async fn get_rates() -> Result<HashMap<String, f64>, &'static str>{
    let fixerio = Fixerio::new(env::var("FIXERIO_ACCESS_KEY").expect("No fixerio access key set."));
    let rates = fixerio.get_rates().await;

    match rates {
        Ok(data) => {
            return Ok(data.rates);
        }
        Err(e) => {
            return Err(e)
        }
    }
}

pub fn convert(from : String, to : String) {
    let rates = get_rates();
}