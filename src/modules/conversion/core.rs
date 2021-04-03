extern crate measurements;

use super::measurement::measurement::{get_units, to_unit};
use super::{currency::currency::get_all_currencies, models};

trait ConversionModule {
    fn to_unit();
    fn get_units();
    fn convert();
}

pub fn get_all_codes_and_symbols() -> Vec<String> {
    let mut values = Vec::new();

    for unit in get_units() {
        values.push(String::from(unit.code).to_lowercase());
        values.push(unit.symbol.to_lowercase());
    }

    for currency in get_all_currencies() {
        values.push(currency.code.to_lowercase());
        values.push(currency.symbol.to_lowercase());
    }

    return values;
}

pub fn clean_value(value: f64) -> String {
    if value % 1.0 == 0.0 {
        return format!("{}", (value as i64));
    }
    return format!("{0:.2}", value);
}

fn convert_conversion_to_str(conversion: &models::Conversion) -> String {
    let mut value: String = String::from("").to_owned();
    value.push_str(clean_value(conversion.value).as_str());
    value.push_str(conversion.unit.symbol.as_str());
    value
}

pub fn get_conversion_result_field(result: &models::ConversionResult) -> (String, String, bool) {
    let mut value_field: String = String::from("").to_owned();

    let mut i = 0;
    for conversion in result.to.iter() {
        if i != 0 {
            value_field.push_str("\n");
        }
        value_field.push_str(convert_conversion_to_str(conversion).as_str());
        i += 1;
    }
    (convert_conversion_to_str(&result.base), value_field, false)
}

pub fn match_conversion() {
    // let re = Regex::new(r"([+-]?\d+(\.\d+)*)(c|f)(?:$|\n| )?").unwrap();

    // for cap in re.captures_iter(&message.content) {
    //     let value = cap[1].parse::<f64>().unwrap_or(0.0).to_owned();
    //     let unit = cap[3].to_lowercase();
    //     let r = core::convert_measurement(value, unit);

    //     match r {
    //         Ok(result) => {
    //             vec.push(get_conversion_result_field(&result));
    //         }
    //         Err(_) => {}
    //     };
    // }
}

pub fn get_embed() {
    // let mut vec = Vec::new();
    // if !vec.is_empty() {
    //     message
    //         .channel_id
    //         .send_message(&ctx, |m| m.embed(|e| e.color(ctx.get_color()).fields(vec)))
    //         .await
    //         .unwrap();
    // }
}
