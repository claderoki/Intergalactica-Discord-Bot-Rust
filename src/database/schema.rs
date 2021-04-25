table! {
    human (id) {
        id -> Integer,
        user_id -> Unsigned<BigInt>,
        gold -> Integer,
        // timezone -> Nullable<Varchar>,
        // date_of_birth -> Nullable<Varchar>,
        // city -> Nullable<Varchar>,
        // country_code -> Nullable<Varchar>,
        // tester -> Bool,
        // currencies -> Nullable<Varchar>,
    }
}

table! {
    pigeon (id) {
        id -> Integer,
        name -> Varchar,
        human_id -> Integer,
        condition -> Varchar,
        experience -> Integer,
        cleanliness -> Integer,
        happiness -> Integer,
        food -> Integer,
        health -> Integer,
        status -> Varchar,
        gender -> Varchar,
    }
}

table! {
    item (id) {
        id -> Integer,
        name -> Varchar,
        code -> Varchar,
        description -> Varchar,
        image_url -> Varchar,
        rarity -> Varchar,
        explorable -> Bool,
        usable -> Bool,
        category_id -> Integer,
        chance -> Integer,
    }
}

table! {
    human_item (id) {
        id -> Integer,
        human_id -> Integer,
        item_id -> Integer,
        amount -> Integer,
        found -> Bool,
    }
}

table! {
    item_category (id) {
        id -> Integer,
        name -> Varchar,
        code -> Varchar,
        parent_id -> Integer,
    }
}

table! {
    currency (id) {
        id -> Integer,
        rate -> Double,
        is_base -> Bool,
        name -> Varchar,
        code -> Varchar,
        symbol -> Varchar,
    }
}

table! {
    measurement (id) {
        id -> Integer,
        rate -> Double,
        is_base -> Bool,
        name -> Varchar,
        code -> Varchar,
        symbol -> Varchar,
        subtype -> Varchar,
    }
}
