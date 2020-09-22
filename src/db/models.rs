use super::schema::task;

#[derive(Insertable)]
#[table_name = "camera"]
pub struct NewCamera<'a> {
    pub name: &'a str,
}
