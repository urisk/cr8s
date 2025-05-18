use serde_json::{json, Value};
use reqwest::{blocking::Client,StatusCode};

pub mod common;
#[test]
fn test_create_crate(){
    let client = Client::new();
    let rustacean = common::create_test_rustacean(&client);

    let response = client.post(format!("{}/crates", common::APP_HOST))
        .json(&json!({
            "rustacean_id": rustacean["id"],
            "code": "foo",
            "name": "Foo crate",
            "version": "0.1",
            "description": "Foo crate description",
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
    let a_crate: Value = response.json().unwrap();
    assert_eq!(a_crate, json!({
        "id": a_crate["id"],
        "code": "foo",
        "name": "Foo crate",
        "version": "0.1",
        "description": "Foo crate description",
        "rustacean_id": rustacean["id"],
        "created_at": a_crate["created_at"],
    }));


    common::delete_test_rustacean(&client, rustacean);
    common::delete_test_crate(&client, a_crate);
}

