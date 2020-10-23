#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde;

use backend::db::{query_camera, establish_connection};
use rocket_contrib::json::Json;
use sentinel;

#[get("/cameras")]
fn cameras_get() -> Json<sentinel::JsonApiResponse> {
    // "this is a response\n".into()
    let mut response = sentinel::JsonApiResponse { data: vec![], };

    let conn = establish_connection();
    for db_camera in query_camera(&conn) {
        let api_camera = sentinel::Camera {
            id: db_camera.id,
            name: db_camera.name,
            address: db_camera.address,
        };
        response.data.push(api_camera);
    }

    Json(response)
}

fn main() {
    rocket::ignite()
        .mount("/", routes![cameras_get])
        .launch();
}