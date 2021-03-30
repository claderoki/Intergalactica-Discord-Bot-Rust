use mysql::{from_row, Row};

use crate::modules::{
    pigeon::{
        models::pigeon::Pigeon,
        repository::pigeon::{create_pigeon, get_active_pigeon},
    },
    shared::repository::human::save_human,
};

#[derive(Debug)]
pub struct Human {
    pub id: i32,
    pub user_id: u64,
    pub gold: i32,
    pub timezone: Option<String>,
    pub date_of_birth: Option<String>,
    pub city: Option<String>,
    pub country_code: Option<String>,
    pub tester: bool,
    pub currencies: Option<String>,
}

type HumanType = (
    i32,
    u64,
    i32,
    Option<String>,
    Option<String>,
    Option<String>,
    Option<String>,
    bool,
    Option<String>,
);

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

    pub fn buy_pigeon(&mut self, name: &str) -> String {
        let cost = 50;

        if name == "" {
            return "No name given.".into();
        }

        if self.gold < cost {
            let s = format!("You need **{}** gold to purchase a pigeon.", cost);
            return s;
        }

        if let Ok(pigeon) = get_active_pigeon(self.id) {
            let s = format!(
                "You already have a lovely pigeon named **{}**.",
                pigeon.name
            );
            return s;
        }

        if let Err(e) = create_pigeon(self.id, name) {
            println!("ERROR, {}", e);
            return "Try again later!".into();
        }

        self.gold -= cost;
        save_human(self);

        format!("You just bought yourself a new pigeon (**-{}**)", cost)
    }
}
