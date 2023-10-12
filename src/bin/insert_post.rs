use diesel::RunQueryDsl;
use utility::{create_conn};
use utility::models::{NewPost, Post};
use utility::schema::posts;

fn main() {
    let mut conn = create_conn();
    let title = "enty";
    let body = "square";
    let new_post = NewPost { title, body };
    diesel::insert_into(posts::table)
        .values(&new_post)
        .get_result::<Post>(&mut conn)
        .expect("Error saving new post");
}
