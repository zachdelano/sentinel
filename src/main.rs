// use sqlite;
use diesel::sqlite;
use sentinel::db;

// querying tasks: https://erwabook.com/intro/create-a-database-access-layer.html#querying-tasks
fn main() {
    // let connection = sqlite::open(":memory:").unwrap();

    // connection
    //     .execute(
    //         "
    //         CREATE TABLE users (name TEXT, age INTEGER);
    //         INSERT INTO users VALUES ('Alice', 42);
    //         INSERT INTO users VALUES ('Bob', 69);
    //         "
    //     )
    //     .unwrap();
    // connection
    //     .iterate("SELECT * FROM users WHERE age > 50", |pairs| {
    //         for &(column, value) in pairs.iter() {
    //             println!("{} = {}", column, value.unwrap());
    //         }
    //         true
    //     })
    //     .unwrap();
}