use super::schema::{crates, rustaceans};
use chrono::NaiveDateTime;
use diesel::prelude::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize)]
pub struct Rustaceans {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub create_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = rustaceans)]
pub struct NewRustaceans {
    pub name: String,
    pub email: String,
}

#[derive(Queryable, Serialize)]
pub struct Crates {
    pub id: i32,
    pub rustaceans_id: i32,
    pub code: String,
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub create_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name=crates)]
pub struct NewCrates {
    pub rustaceans_id: i32,
    pub code: String,
    pub name: String,
    pub version: String,
    pub description: Option<String>,
}
