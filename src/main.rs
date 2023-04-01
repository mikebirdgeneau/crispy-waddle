
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::RunQueryDsl;
use diesel::upsert::excluded;
use dotenvy::dotenv;
use std::env;

// Schema auto generated by diesel.rs:
pub mod schema;


// This is the struct that represents the table in the database:
#[derive(Queryable,Insertable,AsChangeset)]
#[diesel(table_name=schema::posts)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}


// Establish a connection to the database:
pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

// Main entry point:
fn main() {

    let connection = &mut establish_connection();


    // Create a new post:
    let initial_post = Post {
        id: 1,
        title: "Hello World".to_string(),
        body: "This is my first post".to_string(),
        published: false,
    };

    // Insert the new post into the database, but only if it doesn't already exist:
    use schema::posts::dsl::*;
    diesel::insert_into(posts)
        .values(&initial_post)
        .on_conflict(id)
        .do_nothing()
        .execute(connection)
        .expect("Error saving new post");

    // Now, let's create an update to the data that we want to update/insert:
    let post_set = vec![ Post {
        id: 1,
        title: "Hello World".to_string(),
        body: "This is my first post, with some edits".to_string(),
        published: true,
    },
    Post {
        id: 2,
        title: "Hello World 2".to_string(),
        body: "This is my second post".to_string(),
        published: false,
    }];

    // Run update/insert; this works, but I'm wondering if there's a way to make this more DRY:
    let rows_affected = diesel::insert_into(posts)
        .values(&post_set)
        .on_conflict(id)
        .do_update()
        // ==== CAN THIS SECTION BE MORE DRY? ====
        .set((
            id.eq(excluded(id)),
            title.eq(excluded(title)),
            body.eq(excluded(body)),
            published.eq(excluded(published)),
        ))
        // ======================================
        .execute(connection).unwrap();

    println!("{} rows affected", rows_affected);
    println!("");

    // Query the database and print results:
    let results = posts
        .limit(5)
        .load::<Post>(connection)
        .expect("Error loading posts");
    println!("Displaying {} posts:", results.len());
    for post in results {
        println!("{}", post.title);
        println!("------------");
        println!("{}", post.body);
        println!("{}", post.published);
        println!("");
    };
}
