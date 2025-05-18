use crate::repositories::RustaceansRepository;
use rocket::http::Status;
use rocket::response::status::{Custom, NoContent};
use rocket::serde::json::serde_json::json;
use rocket::serde::json::Json;
use rocket::serde::json::Value;
use rocket_db_pools::Connection;
use crate::rocket_routes::DbConn;
use crate::models::{NewRustacean, Rustacean};

#[rocket::get("/rustaceans")]
pub async fn get_rustaceans(mut db: Connection<DbConn>) -> Result<Value, Custom<Value>> {
    RustaceansRepository::find_multiple(&mut db, 100).await
        .map(|rustaceans| json!(rustaceans))
        .map_err(|_| Custom(Status::InternalServerError, json!("Error")))
    // .map_err(|err| {
    //     // Convert the error to a string and include it in the response
    //     let error_message = format!("Database error: {}", err);
    //     Custom(Status::InternalServerError, json!({ "error": error_message }))
    //})
}

#[rocket::get("/rustaceans/<id>")]
pub async fn view_rustacean(mut db: Connection<DbConn>,id: i32) -> Result<Value, Custom<Value>> {
    RustaceansRepository::find(&mut db, id).await
        .map(|rustacean| json!(rustacean))
        .map_err(|_| Custom(Status::InternalServerError, json!("Error")))
}

#[rocket::post("/rustaceans", format="json", data = "<new_rustacean>")]
pub async fn create_rustacean(mut db: Connection<DbConn>, new_rustacean: Json<NewRustacean>) -> Result<Custom<Value>, Custom<Value>> {
    RustaceansRepository::create(&mut db, new_rustacean.into_inner()).await
        .map(|rustacean| Custom(Status::Created, json!(rustacean)))
        .map_err(|_| Custom(Status::InternalServerError, json!("Error")))
}

#[rocket::put("/rustaceans/<id>", format="json", data = "<rustacean>")]
pub async fn update_rustacean(mut db: Connection<DbConn>, id: i32, rustacean: Json<Rustacean>) -> Result<Value, Custom<Value>> {
    RustaceansRepository::update(&mut db, id, rustacean.into_inner()).await
        .map(|rustacean| json!(rustacean))
        .map_err(|_| Custom(Status::InternalServerError, json!("Error")))
}

#[rocket::delete("/rustaceans/<id>")]
pub async fn delete_rustacean(mut db: Connection<DbConn>,id: i32) -> Result<NoContent, Custom<Value>> {
    RustaceansRepository::delete(&mut db, id).await
        .map(|_| NoContent)
        .map_err(|_| Custom(Status::InternalServerError, json!("Error")))
}

