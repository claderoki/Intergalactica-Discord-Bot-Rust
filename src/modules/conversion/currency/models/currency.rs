use mysql::{Row, from_row};


#[derive(Debug)]
pub struct Currency {
    pub id: i32,
    pub rate: f64,
    pub is_base: bool,
    pub name: String,
    pub code: String,
    pub symbol: String
}

impl Currency {
    pub fn from_row(row : Row) -> Currency {
        let values = from_row::<(i32, f64, bool, String, String, String)>(row);
        Currency {
            id: values.0,
            rate: values.1,
            is_base: values.2,
            name: values.3,
            code: values.4,
            symbol: values.5
        }
    }
}