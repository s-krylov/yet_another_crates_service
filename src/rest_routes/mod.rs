use rocket_db_pools::Database;

pub mod crates;
pub mod rustaceans;

#[derive(Database)]
#[database("postgres")]
pub struct DbConnection(rocket_db_pools::diesel::PgPool);
