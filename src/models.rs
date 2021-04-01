use super::chrono;

#[derive(Queryable)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub created_at: Option<chrono::NaiveDate>,
    pub until_at: Option<chrono::NaiveDate>,
    pub in_progress: bool,
}

use super::schema::tasks;

#[derive(Insertable)]
#[table_name="tasks"]
pub struct NewTask<'a> {
    pub title: &'a str,
    pub until_at: &'a chrono::NaiveDate,
}
