#[macro_use]
extern crate serde;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Camera {
    pub id: Option<i32>,
    pub name: String,
    pub address: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct JsonApiResponse {
    pub data: Vec<Camera>,
}