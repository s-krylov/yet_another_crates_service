use crate::common::{APP_HOST, Rezult, create_viewer_rest_client};
use reqwest::StatusCode;
use rocket::serde::json::Value;

mod common;

#[test]
fn test_me() -> Rezult {
    let client_viewer = create_viewer_rest_client()?;
    let response = client_viewer.get(format!("{APP_HOST}/me")).send()?;

    assert_eq!(response.status(), StatusCode::OK);
    let response: Value = response.json()?;
    assert_eq!(response["username"], "dummy_user");
    Ok(())
}
