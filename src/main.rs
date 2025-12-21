use rocket::routes;
use rocket_db_pools::Database;

mod models;
mod repository;
mod rest_routes;
mod schema;

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount(
            "/",
            routes![
                rest_routes::rustaceans::get_rustaceans,
                rest_routes::rustaceans::get_rustacean,
                rest_routes::rustaceans::create_rustacean,
                rest_routes::rustaceans::update_rustacean,
                rest_routes::rustaceans::delete_rustacean,
                rest_routes::crates::get_crates,
                rest_routes::crates::get_crate,
                rest_routes::crates::create_crate,
                rest_routes::crates::update_crate,
                rest_routes::crates::delete_crate,
            ],
        )
        .attach(rest_routes::DbConnection::init())
        .launch()
        .await;
}
