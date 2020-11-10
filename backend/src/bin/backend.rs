#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde;

use std::error::Error;
use rocket_cors::{AllowedHeaders, AllowedOrigins};
use rocket::{get,routes};
use rocket::http::Method;
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

fn main() -> Result<(), Box<dyn Error>> {
    // https://erwabook.com/intro/create-a-browser-based-frontend-ui.html
    let allowed_origins = AllowedOrigins::some_exact(&["http://127.0.0.1:8080"]);

    // You can also deserialize this
    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    }
        .to_cors()?;

    rocket::ignite()
        .mount("/", routes![cameras_get])
        .attach(cors)
        .launch();
    println!("any output");
    Ok(())
}