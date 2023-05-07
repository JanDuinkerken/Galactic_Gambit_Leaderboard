use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use models::{Entries, NewEntry};
use std::env;

pub mod models;
pub mod schema;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_entry(
    conn: &mut PgConnection,
    level_id: &String,
    username: &String,
    seconds: &f64,
) -> Entries {
    use crate::schema::entries;

    let new_entry = NewEntry {
        level_id,
        username,
        seconds,
    };

    diesel::insert_into(entries::table)
        .values(&new_entry)
        .get_result(conn)
        .expect("Error saving new time entry")
}
