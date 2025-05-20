use chrono::NaiveDateTime;
use diesel::{prelude::*};
use crate::schema::*;
use serde::{Serialize,Deserialize};

#[derive(Queryable, AsChangeset, Serialize, Deserialize)]
pub struct Rustacean{
    #[serde(skip_deserializing)]
    pub id: i32,
    pub name: String,
    pub email: String,
    #[serde(skip_deserializing)]
    created_at: NaiveDateTime
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name=rustaceans)]
pub struct NewRustacean{
    pub name: String,
    pub email: String,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct Crate{
    #[serde(skip_deserializing)]
    pub id: i32,
    pub rustacean_id: i32,
    pub code: String,
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    #[serde(skip_deserializing)]
    created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize, Debug)]
#[diesel(table_name=crates)]
pub struct NewCrate{
    pub rustacean_id: i32,
    pub code: String,
    pub name: String,
    pub version: String,
    pub description: Option<String>,
}