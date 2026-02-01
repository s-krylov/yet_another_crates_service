pub mod roles;
pub mod users;

use diesel_async::{AsyncConnection, AsyncPgConnection};
use std::{env::var, str::FromStr};

use crate::models::RoleCodes;

async fn create_db_connection() -> AsyncPgConnection {
    let db_url = var("DATABASE_URL").expect("DATABASE_URL environment variaable should be set");
    AsyncPgConnection::establish(&db_url)
        .await
        .expect("Can't create AsyncConnection from url")
}

fn convert_to_role_codes(roles: &[String]) -> Vec<RoleCodes> {
    roles
        .iter()
        .filter_map(|s| RoleCodes::from_str(s).ok())
        .collect::<Vec<_>>()
}
