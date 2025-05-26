use serde_json::{json, Value};
use reqwest::{blocking::Client, StatusCode};
use common::delete_test_rustacean;

pub mod common;

#[test]
fn test_create_crate() {
    let client = Client::new();

    // Create a rustacean first
    println!("Creating test rustacean...");
    let rustacean = common::create_test_rustacean(&client).clone();
    println!("Rustacean created: {:?}", rustacean);

    // Prepare crate data
    let crate_data = json!({
        "rustacean_id": rustacean["id"],
        "code": "foo",
        "name": "Foo crate",
        "version": "0.1",
        "description": "Foo crate description"
    });
    println!("Sending crate data: {:?}", crate_data);

    // Try to create the crate
    let response = client.post(format!("{}/crates", common::APP_HOST))
        .json(&crate_data)
        .send()
        .unwrap();

    // Display detailed response information
    let status = response.status();
    println!("Response status: {}", status);

    let response_text = response.text().unwrap();
    println!("Response body: {}", response_text);

    // The test will fail at this point if the status is not 201
    assert_eq!(status, StatusCode::CREATED);

    // Parse the response
    let a_crate: Value = serde_json::from_str(&response_text).unwrap();

    // The rest of the test would continue here...

    // Clean up - note we need to recreate the client since we consumed the response
    let client = Client::new();
    delete_test_rustacean(&client, rustacean);
}

