use rocket_db_pools::Database;

pub mod rustaceans;
pub mod crates;

#[derive(Database)]
#[database("postgres")] //database to connect to from compose file?
pub struct DbConn(rocket_db_pools::diesel::PgPool);


