use pretty_assertions::assert_eq;
use reqwest::StatusCode;
use rocket::form::validate::Contains;
use serde_json::{Value, json};

use common::{
    APP_HOST, Rezult, create_admin_rest_client, create_test_crate, create_test_rustecean,
    delete_test_crate, delete_test_rustecean,
};

mod common;

#[test]
fn test_list_crates() -> Rezult {
    let client = create_admin_rest_client()?;
    let rustacean1 = create_test_rustecean(&client)?;
    let rustacean2 = create_test_rustecean(&client)?;
    let a_crate1 = create_test_crate(&client, rustacean1["id"].as_i64().unwrap())?;
    let a_crate2 = create_test_crate(&client, rustacean2["id"].as_i64().unwrap())?;

    let response = client.get(format!("{APP_HOST}/crates")).send()?;

    assert_eq!(response.status(), StatusCode::OK);
    let response: Value = response.json()?;
    let respose = response.as_array();
    assert!(respose.contains(&a_crate1) && respose.contains(&a_crate2));

    let _ = delete_test_crate(&client, a_crate1["id"].as_i64().unwrap())?;
    let _ = delete_test_crate(&client, a_crate2["id"].as_i64().unwrap())?;
    let _ = delete_test_rustecean(&client, rustacean1["id"].as_i64().unwrap())?;
    delete_test_rustecean(&client, rustacean2["id"].as_i64().unwrap())
}

#[test]
fn test_get_crates() -> Rezult {
    let client = create_admin_rest_client()?;
    let rustacean = create_test_rustecean(&client)?;
    let a_crate = create_test_crate(&client, rustacean["id"].as_i64().unwrap())?;

    let response = client
        .get(format!("{APP_HOST}/crates/{}", a_crate["id"]))
        .send()?;

    assert_eq!(response.status(), StatusCode::OK);
    let response: Value = response.json()?;
    assert_eq!(
        response,
        json!({
            "id": a_crate["id"],
            "rustaceans_id": a_crate["rustaceans_id"],
            "code": a_crate["code"],
            "name": a_crate["name"],
            "version": a_crate["version"],
            "description": a_crate["description"],
            "create_at": a_crate["create_at"],
        })
    );

    let _ = delete_test_crate(&client, a_crate["id"].as_i64().unwrap())?;
    delete_test_rustecean(&client, rustacean["id"].as_i64().unwrap())
}

#[test]
fn test_get_crates_not_found() -> Rezult {
    let client = create_admin_rest_client()?;
    let response = client
        .get(format!("{APP_HOST}/crates/{}", 1000000))
        .send()?;

    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    Ok(())
}

#[test]
fn test_create_crates() -> Rezult {
    let client = create_admin_rest_client()?;
    let rustacean = create_test_rustecean(&client)?;
    let response = create_test_crate(&client, rustacean["id"].as_i64().unwrap())?;

    assert_eq!(
        response,
        json!({
            "id": response["id"],
            "rustaceans_id": response["rustaceans_id"],
            "code": response["code"],
            "name": response["name"],
            "version": response["version"],
            "description": response["description"],
            "create_at": response["create_at"],
        })
    );

    let _ = delete_test_crate(&client, response["id"].as_i64().unwrap())?;
    delete_test_rustecean(&client, rustacean["id"].as_i64().unwrap())
}

#[test]
fn test_create_crates_fails_for_not_existing_rustaceans() -> Rezult {
    let client = create_admin_rest_client()?;
    let response = client
        .post(format!("{APP_HOST}/crates"))
        .json(&json!({
            "rustaceans_id": 10000000,
            "code": "crate-code",
            "name": "crate name",
            "version": "1.0.0",
            "email": "john.doel@mail.com"
        }))
        .send()?;

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    Ok(())
}

#[test]
fn test_update_crates() -> Rezult {
    let client = create_admin_rest_client()?;
    let rustacean = create_test_rustecean(&client)?;
    let a_crate = create_test_crate(&client, rustacean["id"].as_i64().unwrap())?;

    let update_response = client
        .put(format!("{APP_HOST}/crates/{}", a_crate["id"]))
        .json(&json!({
            "rustaceans_id": a_crate["rustaceans_id"],
            "code": "yet-another-code",
            "name": "yet-another-name",
            "version": "1.0.1",
            "description": "amazing library"
        }))
        .send()?;

    assert_eq!(update_response.status(), StatusCode::OK);
    let update_response: Value = update_response.json()?;
    assert_eq!(
        update_response,
        json!({
            "id": update_response["id"],
            "rustaceans_id": update_response["rustaceans_id"],
            "code": update_response["code"],
            "name": update_response["name"],
            "version": update_response["version"],
            "description": update_response["description"],
            "create_at": update_response["create_at"]
        })
    );

    let _ = delete_test_crate(&client, update_response["id"].as_i64().unwrap())?;
    delete_test_rustecean(&client, rustacean["id"].as_i64().unwrap())
}

#[test]
fn test_delete_crates() -> Rezult {
    let client = create_admin_rest_client()?;
    let rustacean = create_test_rustecean(&client)?;
    let a_crate = create_test_crate(&client, rustacean["id"].as_i64().unwrap())?;

    let _ = delete_test_crate(&client, a_crate["id"].as_i64().unwrap())?;
    delete_test_rustecean(&client, rustacean["id"].as_i64().unwrap())
}
