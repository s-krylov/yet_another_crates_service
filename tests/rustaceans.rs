use pretty_assertions::assert_eq;
use reqwest::{StatusCode, blocking::Client};
use rocket::form::validate::Contains;
use serde_json::{Value, json};

use common::{APP_HOST, Rezult, create_test_rustecean, delete_test_rustecean};

mod common;

#[test]
fn test_list_rustaceans() -> Rezult {
    let rustacean1 = create_test_rustecean()?;
    let rustacean2 = create_test_rustecean()?;
    let response = Client::new().get(format!("{APP_HOST}/rustaceans")).send()?;

    assert_eq!(response.status(), StatusCode::OK);
    let response: Value = response.json()?;
    let respose = response.as_array();
    assert!(respose.contains(&rustacean1) && respose.contains(&rustacean2));

    let _ = delete_test_rustecean(rustacean1["id"].as_i64().unwrap())?;
    delete_test_rustecean(rustacean2["id"].as_i64().unwrap())
}

#[test]
fn test_get_rustaceans() -> Rezult {
    let rustacean = create_test_rustecean()?;
    let response = Client::new()
        .get(format!("{APP_HOST}/rustaceans/{}", rustacean["id"]))
        .send()?;

    assert_eq!(response.status(), StatusCode::OK);
    let response: Value = response.json()?;

    assert_eq!(
        response,
        json!({
            "id": response["id"],
            "name": rustacean["name"],
            "email": rustacean["email"],
            "create_at": response["create_at"],
        })
    );

    delete_test_rustecean(response["id"].as_i64().unwrap())
}

#[test]
fn test_get_rustaceans_not_found() -> Rezult {
    let response = Client::new()
        .get(format!("{APP_HOST}/rustaceans/{}", 1000000))
        .send()?;

    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    Ok(())
}

#[test]
fn test_create_rustaceans() -> Rezult {
    let json_response: Value = create_test_rustecean()?;
    assert_eq!(
        json_response,
        json!({
            "id": json_response["id"],
            "name": "my super awesome rustacean",
            "email": "john.doel@mail.com",
            "create_at": json_response["create_at"],
        })
    );

    delete_test_rustecean(json_response["id"].as_i64().unwrap())
}

#[test]
fn test_update_rustaceans() -> Rezult {
    let json_response: Value = create_test_rustecean()?;

    let update_response = Client::new()
        .put(format!("{APP_HOST}/rustaceans/{}", json_response["id"]))
        .json(&json!({
            "name": "my amazing new name",
            "email": "john.diesel@mail.com"
        }))
        .send()?;

    assert_eq!(update_response.status(), StatusCode::OK);
    let update_response: Value = update_response.json()?;
    assert_eq!(
        update_response,
        json!({
            "id": update_response["id"],
            "name": "my amazing new name",
            "email": "john.diesel@mail.com",
            "create_at": update_response["create_at"],
        })
    );

    delete_test_rustecean(update_response["id"].as_i64().unwrap())
}

#[test]
fn test_delete_rustaceans() -> Rezult {
    let json_response: Value = create_test_rustecean()?;
    delete_test_rustecean(json_response["id"].as_i64().unwrap())
}
