use std::env::args;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use diesel::prelude::*;
use utility::create_conn;
use utility::models::{Message, Post};
use utility::schema::messages::dsl::messages;
use utility::schema::posts::dsl::{posts, published, title};

fn main() {
    let id = 6;

    let mut conn = create_conn();

    let post = diesel::update(posts.find(id))
        .set((title.eq("enty"), published.eq(true)))
        .get_result::<Post>(&mut conn)
        .unwrap_or_else(|_| panic!("Unable to find post {}", id));
    println!("Published post {}", post.title);

    // let (tx,rx) = mpsc::channel();
    // thread::spawn(move ||{
    //     let v = vec![
    //         String::from("hello1"),
    //         String::from("hello2"),
    //         String::from("hello3"),
    //         String::from("hello4"),
    //     ];
    //
    //     for val in v {
    //         tx.send(val).unwrap();
    //         thread::sleep(Duration::from_secs(3));
    //     }
    // });
    // for received in rx{
    //     println!("Got: {:?}", received);
    // }
}
