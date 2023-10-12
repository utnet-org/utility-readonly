use std::env::args;

use diesel::prelude::*;
use utility::{create_conn, schema::posts::dsl::*};

fn main() {
    // let target = args().nth(1).expect("rust");
    // let pattern = format!("%{}%", target);
    let pattern = format!("%{}%", "enty");

    let mut conn = create_conn();
    let num_deleted = diesel::delete(posts.filter(title.like(pattern)))
        .execute(&mut conn)
        .expect("error deleting posts");

    println!("Deleted {} posts", num_deleted);
}
