use diesel::query_dsl::methods::{FindDsl, LimitDsl};
use diesel::{ExpressionMethods, QueryResult};
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::models::{Crates, NewCrates, NewRustaceans, Rustaceans};
use crate::schema;

pub struct RustaceansRepository;

impl RustaceansRepository {
    pub async fn find_ond(con: &mut AsyncPgConnection, id: i32) -> QueryResult<Rustaceans> {
        schema::rustaceans::table.find(id).get_result(con).await
    }

    pub async fn list(con: &mut AsyncPgConnection, limit: i64) -> QueryResult<Vec<Rustaceans>> {
        schema::rustaceans::table
            .limit(limit)
            .get_results(con)
            .await
    }

    pub async fn create(
        con: &mut AsyncPgConnection,
        rustacean: NewRustaceans,
    ) -> QueryResult<Rustaceans> {
        diesel::insert_into(schema::rustaceans::table)
            .values(rustacean)
            .get_result(con)
            .await
    }

    pub async fn update(
        con: &mut AsyncPgConnection,
        id: i32,
        rustacean: NewRustaceans,
    ) -> QueryResult<Rustaceans> {
        diesel::update(schema::rustaceans::table.find(id))
            .set((
                schema::rustaceans::name.eq(rustacean.name),
                schema::rustaceans::email.eq(rustacean.email),
            ))
            .get_result(con)
            .await
    }

    pub async fn delete(con: &mut AsyncPgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(schema::rustaceans::table.find(id))
            .execute(con)
            .await
    }
}

pub struct CratesRepository;

impl CratesRepository {
    pub async fn find_ond(con: &mut AsyncPgConnection, id: i32) -> QueryResult<Crates> {
        schema::crates::table.find(id).get_result(con).await
    }

    pub async fn list(con: &mut AsyncPgConnection, limit: i64) -> QueryResult<Vec<Crates>> {
        schema::crates::table.limit(limit).get_results(con).await
    }

    pub async fn create(con: &mut AsyncPgConnection, a_crate: NewCrates) -> QueryResult<Crates> {
        diesel::insert_into(schema::crates::table)
            .values(a_crate)
            .get_result(con)
            .await
    }

    pub async fn update(
        con: &mut AsyncPgConnection,
        id: i32,
        a_crate: NewCrates,
    ) -> QueryResult<Crates> {
        diesel::update(schema::crates::table.find(id))
            .set((
                schema::crates::name.eq(a_crate.name),
                schema::crates::code.eq(a_crate.code),
                schema::crates::version.eq(a_crate.version),
                schema::crates::description.eq(a_crate.description),
            ))
            .get_result(con)
            .await
    }

    pub async fn delete(con: &mut AsyncPgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(schema::crates::table.find(id))
            .execute(con)
            .await
    }
}
