mod dao;
mod spec;
mod service;
mod models;
mod schema;
mod test;

use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
use crate::service::course::course::{*};

#[actix_rt::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();
    //读取所有的环境变量
    // for (key, value) in env::vars() {
    //     println!("环境变量内容：{}: {}", key, value);
    // }
    let connection_str = env::var("DATABASE_URL")
        .expect("database connection error");
    println!("db connection info：{}", connection_str);

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&connection_str)
        .await?;
    println!("db_pool is : {:?}", pool);

    // add_course(&pool).await;
    // remove_course(&pool).await;
    // modify_course(&pool).await;
    // get_all_course(&pool).await;
    // get_one_course(&pool).await;
    // get_one_course_as(&pool).await;
    insert_course_tx(&pool).await;

    Ok(())
}


/// 处理操作结果
fn handle_result(result: Result<(), sqlx::Error>, success_message: &str, failure_message: &str) {
    match result {
        Ok(_) => {
            println!("{}", success_message);
        }
        Err(err) => {
            eprintln!("{}: {:?}", failure_message, err);
        }
    }
}




