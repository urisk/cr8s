use serde_json::{json, Value};
use reqwest::{blocking::Client, StatusCode};
use rocket::yansi::Paint;

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
    common::delete_test_crate(&client, a_crate);
    common::delete_test_rustacean(&client, rustacean);

}

#[test]
fn test_view_crate() {
    let client = Client::new();

    // Create a rustacean first
    println!("Creating test rustacean...");
    let rustacean = common::create_test_rustacean(&client);
    let a_crate = common::create_test_crate(&client,&rustacean);

    // Prepare crate data
    let response = client.get(format!("{}/crates/{}", common::APP_HOST, a_crate["id"]))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    // Parse the response
    let a_crate: Value = response.json().unwrap();

    assert_eq!(a_crate, json!(
        {
            "id": a_crate["id"],
            "code": "foo",
            "name": "Foo crate",
            "version": "0.1",
            "description": "Foo crate description",
            "rustacean_id": rustacean["id"],
            "created_at": a_crate["created_at"],
        }));
    // Clean up - note we need to recreate the client since we consumed the response
    common::delete_test_crate(&client, a_crate);
    common::delete_test_rustacean(&client, rustacean);
}

#[test]
fn test_update_crate() {
    let client = Client::new();

    // Create a rustacean first
    println!("Creating test rustacean...");
    let rustacean = common::create_test_rustacean(&client);
    let a_crate = common::create_test_crate(&client,&rustacean);

    // Prepare crate data
    let response = client.put(format!("{}/crates/{}", common::APP_HOST, a_crate["id"]))
        .json(&json!({
            "code": "fooz",
            "name": "Fooz crate",
            "version": "0.2",
            "description": "Fooz crate description",
            "rustacean_id": rustacean["id"],
            "created_at": a_crate["created_at"],
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    //testing author switch
    // let response = client.put(format!("{}/crates/{}", common::APP_HOST, a_crate["id"]))
    //     .json(&json!({
    //         "code": "fooz",
    //         "name": "Fooz crate",
    //         "version": "0.2",
    //         "description": "Fooz crate description",
    //         "rustacean_id": rustacean["id"],
    //         "created_at": 9999,
    //     }))
    //     .send()
    //     .unwrap();
    // assert_eq!(response.status(), StatusCode::OK);

    let a_crate: Value = response.json().unwrap();
    assert_eq!(a_crate, json!({
        "id": a_crate["id"],
        "code": "fooz",
        "name": "Fooz crate",
        "version": "0.2",
        "description": "Fooz crate description",
        "rustacean_id": rustacean["id"],
        "created_at": a_crate["created_at"],
    }));

    // Clean up - note we need to recreate the client since we consumed the response
    common::delete_test_crate(&client, a_crate);
    common::delete_test_rustacean(&client, rustacean);
}

#[test]
fn test_delete_crate() {
    let client = Client::new();

    // Create a rustacean first
    let rustacean = common::create_test_rustacean(&client);
    let a_crate = common::create_test_crate(&client,&rustacean);

    // Prepare crate data
    let response = client.delete(format!("{}/crates/{}", common::APP_HOST, a_crate["id"]))
        .send()
        .unwrap();
    
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
    
    // Clean up - note we need to recreate the client since we consumed the response
    common::delete_test_rustacean(&client, rustacean);
}

#[test]
fn test_get_crates(){
    let client = Client::new();
    let rustacean = common::create_test_rustacean(&client);
    let a_crate = common::create_test_crate(&client, &rustacean);
    let b_crate = common::create_test_crate(&client, &rustacean);

    let response = client.get(format!("{}/crates",common::APP_HOST)).send().unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let json: Value = response.json().unwrap();
    
    assert!(json.as_array().unwrap().contains(&a_crate));
    assert!(json.as_array().unwrap().contains(&b_crate));

    //Clean up
    common::delete_test_crate(&client, a_crate);
    common::delete_test_crate(&client, b_crate);
    common::delete_test_rustacean(&client, rustacean);
}
