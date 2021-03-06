use diesel::{prelude::*, sqlite::SqliteConnection};

pub mod models;
pub mod schema;

pub fn establish_connection() -> SqliteConnection {
    let db = "./sentineldb.sqlite3"; // pull in config to set this
    SqliteConnection::establish(db)
        .unwrap_or_else(|_| panic!("Error connecting to {}", db))
}

pub fn create_camera(connection: &SqliteConnection, name: &str, address: &str) {
    let camera = models::NewCamera { name, address };

    diesel::insert_into(schema::camera::table)
        .values(&camera)
        .execute(connection)
        .expect("Error inserting new camera");
}

pub fn query_camera(connection: &SqliteConnection) -> Vec<models::Camera> {
    schema::camera::table
        .load::<models::Camera>(connection)
        .expect("Error loading cameras")
}