extern crate diesel;
extern crate chrono;
extern crate term_todo;

use diesel::prelude::*;
use self::term_todo::*;

// TODO Add in_progress marker to tasks

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

fn add_task() {}

fn delete_task() {}

fn main() {
    show_tasks();
}
