pub mod course {
    use sqlx::{Acquire, Pool, Postgres};
    use crate::spec::course::course::Course;

    /// 插入数据
    pub async fn insert_course(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
        let insert = sqlx::query!(
        r#"INSERT INTO course ("teacher_id", "name") VALUES ($1, $2)"#,
        10,
        "fuck",
    )
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn insert_course_with(pool: &Pool<Postgres>, cou: Course) -> Result<(), sqlx::Error> {
        // let cou = Course {
        //     teacher_id: 0,
        //     name: "Kobe".to_string(),
        //     ..Default::default()
        // };
        let insert = sqlx::query!(
        r#"INSERT INTO course (teacher_id, name) VALUES ($1, $2)"#,
        cou.teacher_id,
        cou.name,
    )
            .execute(pool)
            .await?;
        Ok(())
    }

    /// 删除数据
    pub async fn delete_course(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
        // sqlx::query("DELETE From course WHERE id=?")
        //     .bind(5)
        //     .execute(pool);
        let delete = sqlx::query!(r#"DELETE FROM course WHERE id=$1"#, 50)
            .execute(pool)
            .await?;
        if delete.rows_affected() == 0 {
            return Err(sqlx::Error::RowNotFound);
        }
        Ok(())
    }

    /// 更新数据
    pub async fn update_course(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
        // 更新
        let update = sqlx::query!(r#"update course set name=$1"#, "enty")
            .execute(pool)
            .await?;
        if update.rows_affected() == 0 {
            return Err(sqlx::Error::RowNotFound);
        }
        Ok(())
    }

    /// 查询所有
    pub async fn query_all_course(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
        let list = sqlx::query!(r#"select * from course"#)
            .fetch_all(pool)
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
        Ok(())
    }

    /// 查询所有
    pub async fn query_one_course(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
        let list2 = sqlx::query!(r#"select * from course where id = $1"#, 1)
            .fetch_all(pool)
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
        println!("查询单个{:?}", vec2);
        Ok(())
    }

    pub async fn insert_tx(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
        let mut conn = pool.acquire().await?;
        let mut tx = conn.begin().await?;

        let result = sqlx::query!(r#"INSERT INTO course ("teacher_id", "name") VALUES ($1, $2)"#,
        10,
        "fuck",
        )
            .execute(&mut tx)
            .await?;

        println!("{:?}", result);
        tx.commit().await?;
        Ok(())
    }

    // 新增并修改（事务）
    pub async fn perform_transaction(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
        // 获取数据库连接
        let mut conn = pool.acquire().await?;

        // 开始事务
        let mut tx = conn.begin().await?;

        // 在事务中执行一系列操作
        let insert_result = sqlx::query!("INSERT INTO course (teacher_id, name) VALUES ($1, $2)", 100, "hehe")
            .execute(&mut tx)
            .await?;

        let update_result = sqlx::query!("UPDATE course SET name = $1 WHERE id = $2", "haha", 11)
            .execute(&mut tx)
            .await?;

        // 检查操作结果，如果有错误则回滚事务
        if insert_result.rows_affected() != 1 || update_result.rows_affected() != 1 {
            tx.rollback().await?;
            return Err(sqlx::Error::RowNotFound);
        }
        // 提交事务
        tx.commit().await?;
        Ok(())
    }
}