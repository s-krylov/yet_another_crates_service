use std::io::Write;
use std::str::FromStr;

use super::schema::{crates, roles, rustaceans, users, users_roles};
use chrono::NaiveDateTime;
use diesel::deserialize::{FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::pg::{Pg, PgValue};
use diesel::prelude::{Associations, Identifiable, Insertable, Queryable, Selectable};
use diesel::serialize::{IsNull, ToSql};
use diesel::sql_types::{VarChar, Varchar};
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
    pub code: RoleCodes,
    pub name: String,
    pub create_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name=roles)]
pub struct NewRoles {
    pub code: RoleCodes,
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

#[derive(AsExpression, Debug, Clone, PartialEq, Eq, FromSqlRow, Serialize, Deserialize)]
#[diesel(sql_type = Varchar)]
pub enum RoleCodes {
    Admin,
    Viewer,
    Editor,
}

impl FromSql<VarChar, Pg> for RoleCodes {
    fn from_sql(value: PgValue) -> diesel::deserialize::Result<Self> {
        match value.as_bytes() {
            b"admin" => Ok(RoleCodes::Admin),
            b"viewer" => Ok(RoleCodes::Viewer),
            b"editor" => Ok(RoleCodes::Editor),
            _ => Ok(RoleCodes::Viewer),
        }
    }
}

impl ToSql<VarChar, Pg> for RoleCodes {
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, Pg>,
    ) -> diesel::serialize::Result {
        let text: &[u8] = match self {
            RoleCodes::Admin => b"admin",
            RoleCodes::Viewer => b"viewer",
            RoleCodes::Editor => b"editor",
        };
        out.write_all(text)?;
        Ok(IsNull::No)
    }
}

impl FromStr for RoleCodes {
    type Err = ();
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "admin" => Ok(RoleCodes::Admin),
            "viewer" => Ok(RoleCodes::Viewer),
            "editor" => Ok(RoleCodes::Editor),
            _ => Err(()),
        }
    }
}

impl ToString for RoleCodes {
    fn to_string(&self) -> String {
        let text = match self {
            RoleCodes::Admin => "admin",
            RoleCodes::Viewer => "viewer",
            RoleCodes::Editor => "editor",
        };
        text.to_string()
    }
}

pub struct EditorUser(pub Users);
