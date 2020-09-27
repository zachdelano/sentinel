use super::schema::camera;

// use diesel::deserialize::{self, FromSql};
// use diesel::backend::Backend;
// use diesel::sql_types::*;

#[derive(Insertable)]
#[table_name = "camera"]
pub struct NewCamera<'a> {
    pub name: &'a str,
    pub address: &'a str
}

#[derive(Queryable)]
pub struct Camera {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub address: Option<String>,
}
