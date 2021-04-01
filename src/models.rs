use super::chrono;

#[derive(Queryable)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub created_at: chrono::NaiveDate,
    pub until_at: Option<chrono::NaiveDate>,
}
