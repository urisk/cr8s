use serde_json::{json, Value};
use reqwest::{blocking::Client,StatusCode};

pub mod common;
#[test]
fn test_get_rustaceans(){
    let client = Client::new();
    let rustacean1 = common::create_test_rustacean(&client);
    let rustacean2 = common::create_test_rustacean(&client);

    let response = client.get(format!("{}/rustaceans",common::APP_HOST)).send().unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let json: Value = response.json().unwrap();
    assert!(json.as_array().unwrap().contains(&rustacean1));
    assert!(json.as_array().unwrap().contains(&rustacean2));

    //Clean up
    common::delete_test_rustacean(&client, rustacean1);
    common::delete_test_rustacean(&client, rustacean2);
}

#[test]
fn test_create_rustaceans(){
    let client = Client::new();
    let response = client.post(format!("{}/rustaceans",common::APP_HOST))
        .json(&json!({
            "name": "Foo Bar",
            "email": "Foo@bar.com",
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    let rustacean: Value = response.json().unwrap();
    assert_eq!(rustacean,json!({
        "id": rustacean["id"],
        "name": "Foo Bar",
        "email": "Foo@bar.com",
        "created_at": rustacean["created_at"],
    }));
    common::delete_test_rustacean(&client, rustacean);
}

#[test]
fn test_view_rustaceans(){
    let client = Client::new();

    let rustacean = common::create_test_rustacean(&client);
    let response = client.get(format!("{}/rustaceans/{}",common::APP_HOST,rustacean["id"]))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let rustacean: Value = response.json().unwrap();
    assert_eq!(rustacean,json!({
        "id": rustacean["id"],
        "name": "Foo Bar",
        "email": "Foo@bar.com",
        "created_at": rustacean["created_at"],
    }));
    common::delete_test_rustacean(&client, rustacean);
}

#[test]
fn test_update_rustaceans(){
    //create a record
    let client = Client::new();
    let rustacean = common::create_test_rustacean(&client);

    //update the record
    let response = client.put(format!("{}/rustaceans/{}",common::APP_HOST,rustacean["id"]))
        .json(&json!({
            "name": "Fooz Bar",
            "email": "Fooz@bar.com",
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let rustacean: Value = response.json().unwrap();
    assert_eq!(rustacean,json!({
        "id": rustacean["id"],
        "name": "Fooz Bar",
        "email": "Fooz@bar.com",
        "created_at": rustacean["created_at"],
    }));
    common::delete_test_rustacean(&client, rustacean);
}

#[test]
fn test_delete_rustaceans(){
    //create a record
    let client = Client::new();
    let rustacean = common::create_test_rustacean(&client);

    //delete the record
    let response = client.delete(format!("{}/rustaceans/{}",common::APP_HOST,rustacean["id"]))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);

}
