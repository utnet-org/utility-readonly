#[macro_use]
extern crate diesel;
use diesel::prelude::*;

pub mod models;
pub mod schema;

pub fn create_conn() -> PgConnection {
    // let connection_str = env::var("DATABASE_URL")
    //     .expect("database connection error");
    let database_url = "postgres://backend:utility123@localhost:8011/utility";
    println!("db connection info：{}", database_url);
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error conn to {}", database_url))
}




