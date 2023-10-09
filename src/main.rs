use chrono::NaiveDate;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
use sqlx::{Pool, Postgres};

#[actix_rt::main]
async fn main() -> Result<(), sqlx::Error> {
    println!("Hello, world!");
    dotenv().ok();
    //读取所有的环境变量
    // for (key, value) in env::vars() {
    //     println!("环境变量内容：{}: {}", key, value);
    // }
    let connection_str = env::var("DATABASE_URL")
        .expect("数据库连接配置获取失败，请检查env文件是否已配置数据库连接字符串");
    println!("数据库连接字符串是：{}", connection_str);
    let pool = PgPoolOptions::new()
        .max_connections(5)
        // .connect("postgres://cml:123456@192.168.1.239:5432/rust_sqlx")
        .connect(&connection_str)
        .await?;
    println!("db_pool is : {:?}", pool);
    //查询所有
    let list = sqlx::query!("select * from course")
        .fetch_all(&pool)
        .await?;
    let mut vec = vec![];
    for row in list {
        vec.push(Course {
            id: row.id,
            teacher_id: row.teacher_id,
            name: row.name,
            time: row.time,
        })
    }
    println!("数据库中的所有数据：{:#?}", vec);
    // 查询单个
    let list2 = sqlx::query!(r#"select * from course where id = $1"#, 1)
        .fetch_all(&pool)
        .await?;
    let mut vec2 = vec![];
    for row in list2 {
        vec2.push(Course {
            id: row.id,
            teacher_id: row.teacher_id,
            name: row.name,
            time: row.time,
        })
    }
    println!("查询单个{:#?}", vec2);
    // 增加
    let res = insert_data(&pool).await;
    match res {
        Ok(()) => {
            // 处理成功的情况
            println!("插入成功");
        }
        Err(err) => {
            // 处理错误的情况
            // eprintln!("插入失败：{}", res.err().unwrap());
            eprintln!("插入失败：{:?}", err);
        }
    }
    // 更新
    let update = sqlx::query!(r#"update course set name=$1"#, "ogg")
        .fetch_all(&pool)
        .await?;
    Ok(())
}

/// 插入数据
pub async fn insert_data(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    let insert = sqlx::query!(
        r#"INSERT INTO course ("teacher_id", "name") VALUES ($1, $2)"#,
        10,
        "fuck",
    )
        .execute(pool)
        .await?;
    Ok(())
}

#[derive(Debug)]
pub struct Course {
    pub id: i32,
    pub teacher_id: i32,
    pub name: String,
    pub time: Option<NaiveDate>,
}

