use std::env::args;

use diesel::prelude::*;
use utility::create_conn;
use utility::models::Post;
use utility::schema::posts::dsl::{posts, published, title};

fn main() {
    let id = 2;

    let mut conn = create_conn();

    let post = diesel::update(posts.find(id))
        .set((title.eq("enty"), published.eq(true)))
        .get_result::<Post>(&mut conn)
        .unwrap_or_else(|_| panic!("Unable to find post {}", id));

    println!("Published post {}", post.title);
}
