#[macro_use]
extern crate diesel;
extern crate chrono;
extern crate term_todo;

mod models;

use diesel::prelude::*;
use self::term_todo::*;
use std::io::stdin;
use std::env::args;
use models::Task;

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

fn delete_task() {
    use schema::tasks::dsl::*;

    let target = args().nth(1).expect("Expected target title");
    let pattern = format!("%{}%", target);
    let conn = establish_connection();

    let num_deleted = diesel::delete(tasks.filter(title.like(pattern)))
        .execute(&conn)
        .expect("Failed to delete!");
    println!("Deleted {} posts", num_deleted);
}

fn update_data() {
    use schema::tasks::dsl::*;
    
    // Get command line arguments
    let target_id = args().nth(1).expect("publish_post requires post id")
        .parse::<i32>().expect("Invalid ID");
    let conn = establish_connection();
    
    // Update specified post
    let task: Task = diesel::update(tasks.find(target_id))
        .set(in_progress.eq(true))
        .get_result(&conn)
        .expect(&format!("Unable to find task {}", target_id));

    println!("Changed status of task {} to in progress", task.title);


}

fn main() {

    add_task();
    show_tasks();
    println!("-----------------------------------------------------");
    update_data();
    show_tasks();
}
