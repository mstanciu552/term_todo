#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate chrono;

pub mod schema;
pub mod models;

use schema::tasks;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use models::{Task, NewTask, NewTaskEmpty};
use chrono::NaiveDate;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be valid");

    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
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
        let new_task = NewTaskEmpty {
            title
        };

        diesel::insert_into(tasks::table)
            .values(&new_task)
            .get_result(conn)
            .expect("Failed to upload task with no due date")
    }
}
