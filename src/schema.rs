// @generated automatically by Diesel CLI.

diesel::table! {
    crates (id) {
        id -> Int4,
        rustaceans_id -> Int4,
        #[max_length = 64]
        code -> Varchar,
        #[max_length = 128]
        name -> Varchar,
        #[max_length = 64]
        version -> Varchar,
        description -> Nullable<Text>,
        create_at -> Timestamp,
    }
}

diesel::table! {
    rustaceans (id) {
        id -> Int4,
        name -> Varchar,
        email -> Varchar,
        create_at -> Timestamp,
    }
}

diesel::joinable!(crates -> rustaceans (rustaceans_id));

diesel::allow_tables_to_appear_in_same_query!(
    crates,rustaceans,);
