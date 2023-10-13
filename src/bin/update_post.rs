use std::env::args;

use diesel::prelude::*;
use utility::create_conn;
use utility::models::{Message, Post};
use utility::schema::messages::dsl::messages;
use utility::schema::posts::dsl::{posts, published, title};

fn main() {
    let id = 3;

    let mut conn = create_conn();

    let post = diesel::update(posts.find(id))
        .set((title.eq("enty"), published.eq(true)))
        .get_result::<Post>(&mut conn)
        .unwrap_or_else(|_| panic!("Unable to find post {}", id));
    println!("Published post {}", post.title);

    // let msg = diesel::update(messages.find(id))
    //     .set((title.eq("enty"), published.eq(true)))
    //     .get_result::<Message>(&mut conn)
    //     .unwrap_or_else(|_| panic!("Unable to find msg {}", id));
    // println!("Published msg {}", msg.title);
}
