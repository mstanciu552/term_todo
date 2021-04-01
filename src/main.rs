extern crate diesel;
extern crate chrono;
extern crate term_todo;

use diesel::prelude::*;
use self::term_todo::*;
use std::io::{stdin, Read};

fn show_tasks() {
    use term_todo::schema::tasks::dsl::*;
    use term_todo::models::Task;

    let conn = establish_connection();
    let res: Vec<Task> = tasks.limit(5)
        .load::<Task>(&conn)
        .expect("Error loading tasks");

    println!("There are {} tasks", res.len());
    for task in res {
        println!("Title: {}", task.title);
    }
}

fn add_task() {
    let conn = establish_connection();

    println!("Title: ");
    let mut title = String::new();
    stdin().read_line(&mut title).unwrap();
    let title = title.trim_end();

    println!("Until(YYYY-MM-DD || <empty>): ");
    let mut until_at = String::new();
    stdin().read_line(&mut until_at).unwrap();
    let until_at = until_at.trim_end();

    let task = make_task(&conn, title, until_at);
    println!("Saved task {}", task.title);
}

fn delete_task() {}

fn main() {
    add_task();
    show_tasks();

}
