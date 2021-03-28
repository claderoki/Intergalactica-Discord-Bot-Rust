use mysql::{from_row, Row};

pub struct Human {
    pub id: i32,
    pub user_id: i64,
    pub gold: i32,
    pub timezone: String,
    pub date_of_birth: String,
    pub city: String,
    pub country_code: String,
    pub tester: bool,
    pub currencies: String,
}

type HumanType = (i32, i64, i32, String, String, String, String, bool, String);

impl Human {
    pub fn from_row(row: Row) -> Human {
        let values = from_row::<HumanType>(row);
        Human {
            id: values.0,
            user_id: values.1,
            gold: values.2,
            timezone: values.3,
            date_of_birth: values.4,
            city: values.5,
            country_code: values.6,
            tester: values.7,
            currencies: values.8,
        }
    }
}
