use self::models::{Human, NewHuman};

pub fn create_human<'a>(conn: &PgConnection, user_id : i64) -> Human {
    use schema::human;

    let new_human = NewHuman {
        user_id: user_id,
    };

    diesel::insert_into(human::table)
        .values(&new_human)
        .get_result(conn)
        .expect("Error saving new post")
}