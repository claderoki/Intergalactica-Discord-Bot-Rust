extern crate measurements;

use regex::Regex;

use super::{models::core::{Conversion, ConversionResult}, repository::currency_repository::CurrencyRepository};

trait ConversionModule {
    fn to_unit();
    fn get_units();
    fn convert();
}

pub fn get_all_codes_and_symbols() -> Vec<String> {
    let mut values = Vec::new();

    // for unit in get_units() {
    //     if !values.contains(&unit.code.to_lowercase()) {
    //         values.push(String::from(unit.code).to_lowercase());
    //         values.push(unit.symbol.to_lowercase());
    //     }
    // }

    if let Ok(currencies) = CurrencyRepository::get_all() {
        for currency in currencies {
            if !values.contains(&currency.code.to_lowercase()) {
                values.push(currency.code.to_lowercase());
                values.push(currency.symbol.to_lowercase());
            }
        }
    }

    return values;
}

pub fn clean_value(value: f64) -> String {
    if value % 1.0 == 0.0 {
        return format!("{}", (value as i64));
    }
    return format!("{0:.2}", value);
}

fn convert_conversion_to_str(conversion: &Conversion) -> String {
    let mut value: String = String::from("").to_owned();
    value.push_str(clean_value(conversion.value).as_str());
    value.push_str(conversion.unit.symbol.as_str());
    value
}

pub fn get_conversion_result_field(result: &ConversionResult) -> (String, String, bool) {
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

pub fn get_regex() -> String {
    let mut regex = String::from(r"([+-]?\d+(\.\d+)*)(");

    let mut i = 0;
    for mut value in get_all_codes_and_symbols() {
        if value == "$" || value.contains(".") || value == "" {
            continue;
        }

        value = str::replace(value.as_str(), "/", "\\/");

        regex.push_str(value.as_str());
        if i != 0 {
            regex.push_str("|");
        }
        i += 1;
    }
    regex.push_str(r")(?:$|\n| )?");

    regex
}

pub fn match_conversion(content: &str) {
    //save_all_units();
    //return ();

    // println!("{}", get_regex().as_str());
    let re = Regex::new(get_regex().as_str()).unwrap();

    for cap in re.captures_iter(content) {
        println!("{:?}", cap);
        let value = cap[1].parse::<f64>().unwrap_or(0.0).to_owned();
        let unit = cap[3].to_lowercase();
        println!("{:?}, {:?}", value, unit);
        //     let r = core::convert_measurement(value, unit);

        //     match r {
        //         Ok(result) => {
        //             vec.push(get_conversion_result_field(&result));
        //         }
        //         Err(_) => {}
        //     };
    }
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
