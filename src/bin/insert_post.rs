use diesel::RunQueryDsl;
use utility::{create_conn};
use utility::models::{NewPost, Message, Post, NewMessage};
use utility::schema::{messages, posts};

fn main() {
    let mut conn = create_conn();
    let title = "enty";
    let body = "square";
    // let new = NewPost { title, body };
    let new = NewMessage { title, body };
    diesel::insert_into(messages::table)
        .values(&new)
        .get_result::<Message>(&mut conn)
        .expect("Error saving new post");
}
