use diesel::prelude::*;
use utility::create_conn;
use utility::models::Post;
use utility::schema::posts::dsl::*;

fn main() {
    let pattern = format!("%{}%", "enty");
    let mut conn = create_conn();
    let res = posts
        .filter(published.eq(false))
        .filter(title.like(pattern))
        .limit(2)
        .load::<Post>(&mut conn)
        .expect("Error loading posts");

    println!("Displaying {} posts", res.len());
    for post in res {
        println!("{}", post.title);
        println!("-----------\n");
        println!("{}", post.body);
    }
}