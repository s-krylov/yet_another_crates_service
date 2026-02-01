use common::{APP_HOST, Rezult};
use pretty_assertions::assert_eq;
use reqwest::{StatusCode, blocking::Client};
use rocket::form::validate::Len;
use serde_json::{Value, json};
use std::process::Command;

mod common;

#[test]
fn test_login() -> Rezult {
    let output = Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg("cli")
        .arg("users")
        .arg("create")
        .arg("dummy_user")
        .arg("123")
        .arg("viewer")
        .output()
        .unwrap();

    println!("output = {output:?}");

    let response = Client::new()
        .post(format!("{APP_HOST}/login"))
        .json(&json!({
            "username": "dummy_user",
            "password": "123"
        }))
        .send()?;

    assert_eq!(response.status(), StatusCode::OK);
    let json: Value = response.json().unwrap();

    assert!(json.get("session_id").is_some());
    assert_eq!(json["session_id"].as_str().len(), 128);
    Ok(())
}

#[test]
fn test_login_failed() -> Rezult {
    let output = Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg("cli")
        .arg("users")
        .arg("create")
        .arg("dummy_user")
        .arg("123")
        .arg("viewer")
        .output()
        .unwrap();

    println!("output = {output:?}");

    let response = Client::new()
        .post(format!("{APP_HOST}/login"))
        .json(&json!({
            "username": "defenetly_not_an_user",
            "password": "wewewe"
        }))
        .send()?;

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    Ok(())
}
