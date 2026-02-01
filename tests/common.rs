use reqwest::blocking::{Client, ClientBuilder};
use reqwest::header::{AUTHORIZATION, HeaderMap, HeaderValue};
use reqwest::{Error, StatusCode};
use serde_json::{Value, json};
use std::process::Command;

pub const APP_HOST: &'static str = "http://127.0.0.1:8000";
pub type Rezult = Result<(), reqwest::Error>;

pub fn create_user_rest_client(user: &str, role: &str) -> Result<Client, Error> {
    let output = Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg("cli")
        .arg("users")
        .arg("create")
        .arg(user)
        .arg("123")
        .arg(role)
        .output()
        .unwrap();

    println!("output = {output:?}");

    let response = Client::new()
        .post(format!("{APP_HOST}/login"))
        .json(&json!({
            "username": user,
            "password": "123"
        }))
        .send()?;

    assert_eq!(response.status(), StatusCode::OK);
    let json: Value = response.json().unwrap();

    assert!(json.get("session_id").is_some());
    let session_id = json["session_id"].as_str().unwrap();
    assert_eq!(session_id.len(), 128);

    let auth_header = format!("Bearer {}", session_id);
    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&auth_header[..]).unwrap(),
    );
    Ok(ClientBuilder::new()
        .default_headers(headers)
        .build()
        .unwrap())
}

pub fn create_viewer_rest_client() -> Result<Client, Error> {
    create_user_rest_client("dummy_user", "viewer")
}

pub fn create_editor_rest_client() -> Result<Client, Error> {
    create_user_rest_client("dummy_editor", "editor")
}

pub fn create_test_rustecean(client: &Client) -> Result<Value, Error> {
    let response = client
        .post(format!("{APP_HOST}/rustaceans"))
        .json(&json!({
            "name": "my super awesome rustacean",
            "email": "john.doel@mail.com"
        }))
        .send()?;

    assert_eq!(response.status(), StatusCode::CREATED);
    response.json()
}

pub fn delete_test_rustecean(client: &Client, id: i64) -> Rezult {
    let response = client
        .delete(format!("{APP_HOST}/rustaceans/{id}"))
        .send()?;

    assert_eq!(response.status(), StatusCode::NO_CONTENT);
    Ok(())
}

pub fn create_test_crate(client: &Client, rustacean_id: i64) -> Result<Value, Error> {
    let response = client
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

pub fn delete_test_crate(client: &Client, id: i64) -> Rezult {
    let response = client.delete(format!("{APP_HOST}/crates/{id}")).send()?;

    assert_eq!(response.status(), StatusCode::NO_CONTENT);
    Ok(())
}
