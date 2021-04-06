use super::chrono;
use super::schema::tasks;

#[derive(Queryable)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub created_at: chrono::NaiveDate,
    pub until_at: Option<chrono::NaiveDate>,
    pub in_progress: Option<bool>,
}

#[derive(Insertable)]
#[table_name = "tasks"]
pub struct NewTask<'a> {
    pub title: &'a str,
    pub until_at: Option<&'a chrono::NaiveDate>,
}

#[derive(Insertable)]
#[table_name = "tasks"]
pub struct NewTaskEmpty<'a> {
    pub title: &'a str,
}
