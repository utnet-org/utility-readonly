pub mod course {
    use sqlx::{Pool, Postgres};
    use crate::dao::course::course::{*};
    use crate::handle_result;
    use crate::spec::course::course::Course;

    pub async fn get_all_course(pool: &Pool<Postgres>) -> () {
        let res_all = query_all_course(&pool).await;
        handle_result(res_all, "查询成功", "查询失败");
    }

    pub async fn get_one_course(pool: &Pool<Postgres>) -> () {
        let res_all = query_one_course(&pool).await;
        handle_result(res_all, "查询成功", "查询失败");
    }

    pub async fn get_one_course_as(pool: &Pool<Postgres>) -> () {
        let res_all = query_as(&pool).await;
        handle_result(res_all, "查询成功", "查询失败");
    }

    pub async fn add_course(pool: &Pool<Postgres>) -> () {
        let cou = Course {
            teacher_id: 0,
            name: "Kobe".to_string(),
            ..Default::default()
        };
        let res_all = insert_course_with(&pool, cou).await;
        // let res_all = insert_course(&pool).await;
        handle_result(res_all, "插入成功", "插入失败");
    }


    pub async fn remove_course(pool: &Pool<Postgres>) -> () {
        let res_all = delete_course(&pool).await;
        handle_result(res_all, "删除成功", "删除失败");
    }

    pub async fn modify_course(pool: &Pool<Postgres>) -> () {
        let res_all = update_course(&pool).await;
        handle_result(res_all, "更新成功", "更新失败");
    }

    pub async fn insert_course_tx(pool: &Pool<Postgres>) -> () {
        let insert_tx = perform_transaction(&pool).await;
        handle_result(insert_tx, "插入成功", "插入失败");
    }
}