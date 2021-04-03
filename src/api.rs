extern crate chrono;
extern crate diesel;
extern crate term_todo;

use self::term_todo::*;
use diesel::prelude::*;
use diesel::PgConnection;
use models::Task;
use std::env::args;
use std::io::stdin;

pub struct Database {
    conn: PgConnection,
}

impl Database {
    pub fn new(conn: PgConnection) -> Database {
        Database { conn }
    }
    // Display all tasks
    pub fn show_tasks(&self) {
        use term_todo::schema::tasks::dsl::*;

        let conn = &self.conn;
        let res: Vec<Task> = tasks
            .limit(5)
            .load::<Task>(conn)
            .expect("Error loading tasks");

        println!("There are {} tasks", res.len());
        for task in res {
            println!("-------------------------------------------------");
            println!(
                "|{}: {} ==> {}\t\t\t\t|\n| Due date: {:#?}\t\t\t\t|",
                task.id, task.title, task.in_progress, task.until_at
            );
            println!("-------------------------------------------------");
        }
    }
    // Make a new task
    pub fn add_task(&self) {
        let conn = &self.conn;

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

    pub fn delete_task(&self, arg: bool) {
        use schema::tasks::dsl::*;
        let conn = &self.conn;
        self.show_tasks();
        // Get task to delete based on argument
        if arg {
            let target = args().nth(2).expect("Expected target title");
            let pattern = format!("%{}%", target);
            let num_deleted = diesel::delete(tasks.filter(title.like(pattern)))
                .execute(conn)
                .expect("Failed to delete!");
            println!("Deleted {} posts", num_deleted);

        // Get task to delete based on input from keyboard
        } else {
            let mut target = String::new();
            println!("Task to delete: ");
            stdin().read_line(&mut target).unwrap();
            let target = target.trim_end();
            let pattern = format!("%{}%", target);

            let num_deleted = diesel::delete(tasks.filter(title.like(pattern)))
                .execute(conn)
                .expect("Failed to delete!");
            println!("Deleted {} posts", num_deleted);
        }
        self.show_tasks();
    }
    pub fn update_data(&self, arg: bool) {
        use schema::tasks::dsl::*;
        // Get connection to db
        let conn = &self.conn;
        // Get task to change status based on argument
        if arg {
            // Get command line arguments
            let target_id = args()
                .nth(1)
                .expect("requires task id")
                .parse::<i32>()
                .expect("Invalid ID");

            // Update specified post
            let task: Task = diesel::update(tasks.find(target_id))
                .set(in_progress.eq(true))
                .get_result(conn)
                .expect(&format!("Unable to find task {}", target_id));
            println!("Changed status of task {} to in progress", task.title);
        // Get task to change status based on keyboard input
        } else {
            // Get keyboard input
            println!("Target_id: ");
            let mut target_id = String::new();
            stdin().read_line(&mut target_id).unwrap();
            let target_id = target_id.trim_end().parse::<i32>().unwrap();

            // Update specified post
            let task: Task = diesel::update(tasks.find(target_id))
                .set(in_progress.eq(true))
                .get_result(conn)
                .expect(&format!("Unable to find task {}", target_id));
            println!("Changed status of task {} to in progress", task.title);
        }
    }
}
