#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde;

use backend::db::models::Camera;
use backend::db::{query_camera, establish_connection};
use rocket_contrib::json::Json;

#[derive(Serialize)]
struct JsonApiResponse {
    data: Vec<Camera>
}

#[get("/cameras")]
fn cameras_get() -> Json<JsonApiResponse> {
    // "this is a response\n".into()
    let mut response = JsonApiResponse { data: vec![], };

    let conn = establish_connection();
    for camera in query_camera(&conn) {
        response.data.push(camera);
    }

    Json(response)
}

fn main() {
    rocket::ignite()
        .mount("/", routes![cameras_get])
        .launch();
}