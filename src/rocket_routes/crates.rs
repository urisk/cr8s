use rocket::http::Status;
use rocket::response::status::{Custom, NoContent};
use rocket::serde::json::serde_json::json;
use rocket::serde::json::{Json, Value};
use rocket_db_pools::Connection;
use crate::rocket_routes::DbConn;
use crate::models::{Crate, NewCrate};
use crate::repositories::{CrateRepository};

#[rocket::get("/crates")]
pub async fn get_crates(mut db: Connection<DbConn>) -> Result<Value, Custom<Value>> {
    CrateRepository::find_multiple(&mut db, 100).await
        .map(|crates| json!(crates))
        .map_err(|err| {
            // Convert the error to a string and include it in the response
            let error_message = format!("Database error: {}", err);
            Custom(Status::InternalServerError, json!({ "error": error_message }))
        })
}

#[rocket::get("/crates/<id>")]
pub async fn view_crate(mut db: Connection<DbConn>,id: i32) -> Result<Value, Custom<Value>> {
    CrateRepository::find(&mut db, id).await
        .map(|a_crate| json!(a_crate))
        .map_err(|err| {
            let error_message = format!("Database error: {}", err);
            Custom(Status::InternalServerError, json!({ "error": error_message }))
        })
}

#[rocket::post("/crates", format="json", data = "<new_crate>")]
pub async fn create_crate(mut db: Connection<DbConn>, new_crate: Json<NewCrate>) -> Result<Custom<Value>, Custom<Value>> {
    CrateRepository::create(&mut db, new_crate.into_inner()).await
        .map(|a_crate| Custom(Status::Created, json!(a_crate)))
        .map_err(|err| {
            // Add detailed error message
            let error_message = format!("Database error: {}", err);
            Custom(Status::InternalServerError, json!({ "error": error_message }))
        })
}

#[rocket::put("/crates/<id>", format="json", data = "<a_crate>")]
pub async fn update_crate(mut db: Connection<DbConn>, id: i32, a_crate: Json<Crate>) -> Result<Value, Custom<Value>> {
    CrateRepository::update(&mut db, id, a_crate.into_inner()).await
        .map(|a_crate| json!(a_crate))
        .map_err(|err| {
            rocket::error!("Database error: {}", err);
            let error_message = format!("Database error: {}", err);
            Custom(Status::InternalServerError, json!({ "error": error_message }))
        })
}

#[rocket::delete("/crates/<id>")]
pub async fn delete_crate(mut db: Connection<DbConn>,id: i32) -> Result<NoContent, Custom<Value>> {
    CrateRepository::delete(&mut db, id).await
        .map(|_| NoContent)
        .map_err(|err| {
            rocket::error!("Database error: {}", err);
            Custom(Status::InternalServerError, json!( "Error"))
        })
}