use super::schema::{crates, roles, rustaceans, users, users_roles};
use chrono::NaiveDateTime;
use diesel::prelude::{Associations, Identifiable, Insertable, Queryable, Selectable};
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

#[derive(Queryable, Serialize, Identifiable, Selectable, Debug)]
#[diesel(table_name=users)]
pub struct Users {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub create_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name=users)]
pub struct NewUsers {
    pub username: String,
    pub password: String,
}

#[derive(Queryable, Serialize, Identifiable, Selectable, Debug, Clone)]
#[diesel(table_name=roles)]
pub struct Roles {
    pub id: i32,
    pub code: String,
    pub name: String,
    pub create_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name=roles)]
pub struct NewRoles {
    pub code: String,
    pub name: String,
}

#[derive(Queryable, Serialize, Associations, Identifiable, Debug)]
#[diesel(belongs_to(Users))]
#[diesel(belongs_to(Roles))]
#[diesel(table_name=users_roles)]
pub struct UsersRoles {
    pub id: i32,
    pub users_id: i32,
    pub roles_id: i32,
    pub create_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name=users_roles)]
pub struct NewUsersRoles {
    pub users_id: i32,
    pub roles_id: i32,
}
