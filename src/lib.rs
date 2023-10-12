#[macro_use]
extern crate diesel; //复用diesel里面的宏

use std::env;
use crate::models::{NewPost, Post};
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

// pub fn insert_post(conn: &PgConnection, title: &str, body: &str) -> Post {
//     use schema::posts;
//     let new_post = NewPost { title, body };
//     //下面代码提示会出现问题
//     diesel::insert_into(posts::table)
//         .values(&new_post)
//         .get_result::<Post>(conn)
//         .expect("Error saving new post")
// }



