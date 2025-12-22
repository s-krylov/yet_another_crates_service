use reqwest::blocking::Client;
use reqwest::{Error, StatusCode};
use serde_json::{Value, json};

pub const APP_HOST: &'static str = "http://127.0.0.1:8000";
pub type Rezult = Result<(), reqwest::Error>;

pub fn create_test_rustecean() -> Result<Value, Error> {
    let response = Client::new()
        .post(format!("{APP_HOST}/rustaceans"))
        .json(&json!({
            "name": "my super awesome rustacean",
            "email": "john.doel@mail.com"
        }))
        .send()?;

    assert_eq!(response.status(), StatusCode::CREATED);
    response.json()
}

pub fn delete_test_rustecean(id: i64) -> Rezult {
    let response = Client::new()
        .delete(format!("{APP_HOST}/rustaceans/{id}"))
        .send()?;

    assert_eq!(response.status(), StatusCode::NO_CONTENT);
    Ok(())
}

pub fn create_test_crate(rustacean_id: i64) -> Result<Value, Error> {
    let response = Client::new()
        .post(format!("{APP_HOST}/crates"))
        .json(&json!({
            "rustaceans_id": rustacean_id,
            "code": "crate-code",
            "name": "crate name",
            "version": "1.0.0",
            "email": "john.doel@mail.com"
        }))
        .send()?;

    assert_eq!(response.status(), StatusCode::CREATED);
    response.json()
}

pub fn delete_test_crate(id: i64) -> Rezult {
    let response = Client::new()
        .delete(format!("{APP_HOST}/crates/{id}"))
        .send()?;

    assert_eq!(response.status(), StatusCode::NO_CONTENT);
    Ok(())
}
