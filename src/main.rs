
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::RunQueryDsl;
use dotenvy::dotenv;
use std::env;

pub mod models;
pub mod schema;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}


fn main() {

    let connection = &mut establish_connection();

    let initial_post = models::Post {
        id: 1,
        title: "Hello World".to_string(),
        body: "This is my first post".to_string(),
        published: false,
    };

    use schema::posts::dsl::*;
    diesel::insert_into(posts)
        .values(&initial_post)
        .on_conflict(id)
        .do_nothing()
        .execute(connection)
        .expect("Error saving new post");

    let mut post_set: Vec<models::Post> = Vec::new();
    post_set.push(models::Post {
        id: 1,
        title: "Hello World".to_string(),
        body: "This is my first post, with some edits".to_string(),
        published: true,
    });
    post_set.push(models::Post {
        id: 2,
        title: "Hello World 2".to_string(),
        body: "This is my second post".to_string(),
        published: false,
    });

    // Now, we want to perform an upsert with the new data in post_set, BUT
    // let's imagine a use_case where we have 1000s of rows, and 100+ columns
    // We don't want to have to specify all 100+ columns in the do_update() call
    // Is there a way to do this?

    // This is the query I'm looking for:
    // INSERT INTO posts AS t (
    //    id,
    //    title,
    //    body,
    //    published
    //    ) VALUES (
    //    1,
    //    'Hello World',
    //    'This is my first post, with some edits',
    //    true),
    //    (2,
    //    'Hello World 2',
    //    'This is my second post',
    //    false)
    //    ON CONFLICT (id) DO UPDATE SET (
    //    id,
    //    title,
    //    body,
    //    published
    //    ) = ROW (excluded.*)
    //    WHERE (t.id) IS DISTINCT FROM (excluded.*);

    diesel::insert_into(posts)
        .values(&post_set)
        .on_conflict(id)
        .do_update()
        .set(&post_set) // <-- This is where I want to do a set() with all the columns...
        .execute(connection)
        .expect("Error saving new post");

}
