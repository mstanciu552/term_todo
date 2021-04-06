#[macro_use]
extern crate diesel;
extern crate chrono;
extern crate dotenv;

mod config;
pub mod models;
pub mod schema;

use chrono::NaiveDate;
use config::database_connection;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use models::{NewTask, NewTaskEmpty, Task};
use schema::tasks;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").unwrap_or(database_connection());

    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn make_task<'a>(conn: &PgConnection, title: &'a str, until_at: &'a str) -> Task {
    // Expected date format `YYYY-MM-DD`
    if until_at.len() != 0 {
        let date_separated: Vec<&str> = until_at.split("-").collect();
        let year: i32 = date_separated[0].parse::<i32>().unwrap();
        let month: u32 = date_separated[1].parse::<u32>().unwrap();
        let day: u32 = date_separated[2].parse::<u32>().unwrap();
        let until_date = NaiveDate::from_ymd(year, month, day);
        let new_task = NewTask {
            title,
            until_at: Some(&until_date),
        };
        diesel::insert_into(tasks::table)
            .values(&new_task)
            .get_result(conn)
            .expect("Failed to upload task")
    } else {
        let new_task = NewTaskEmpty { title };

        diesel::insert_into(tasks::table)
            .values(&new_task)
            .get_result(conn)
            .expect("Failed to upload task with no due date")
    }
}
