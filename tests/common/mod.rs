use reqwest::blocking::Client;
use reqwest::StatusCode;
use serde_json::{json,Value};

pub static APP_HOST: &'static str = "http://app:8000";
pub fn create_test_rustacean(client: &Client) -> Value{
    let response = client.post(format!("{}/rustaceans", APP_HOST))
        .json(&json!({
            "name": "Foo Bar",
            "email": "Foo@bar.com",
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    response.json().unwrap()
}

pub fn delete_test_rustacean(client: &Client, rustacean: Value){
    let response = client.delete(format!("{}/rustaceans/{}",APP_HOST,rustacean["id"]))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}

pub fn delete_test_crate(client: &Client, a_crate: Value) {
    let response = client.delete(format!("{}/crates/{}",APP_HOST,a_crate["id"]))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}