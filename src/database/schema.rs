table! {
    human (id) {
        id -> Integer,
        user_id -> Unsigned<BigInt>,
        gold -> Integer,
        timezone -> Nullable<Varchar>,
        date_of_birth -> Nullable<Varchar>,
        city -> Nullable<Varchar>,
        country_code -> Nullable<Varchar>,
        tester -> Bool,
        currencies -> Nullable<Varchar>,
    }
}
