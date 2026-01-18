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
    roles (id) {
        id -> Int4,
        #[max_length = 64]
        code -> Varchar,
        #[max_length = 128]
        name -> Varchar,
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

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 64]
        username -> Varchar,
        #[max_length = 128]
        password -> Varchar,
        create_at -> Timestamp,
    }
}

diesel::table! {
    users_roles (id) {
        id -> Int4,
        users_id -> Int4,
        roles_id -> Int4,
        create_at -> Timestamp,
    }
}

diesel::joinable!(crates -> rustaceans (rustaceans_id));
diesel::joinable!(users_roles -> roles (roles_id));
diesel::joinable!(users_roles -> users (users_id));

diesel::allow_tables_to_appear_in_same_query!(
    crates,roles,rustaceans,users,users_roles,);
