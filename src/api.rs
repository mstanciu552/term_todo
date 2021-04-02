extern crate diesel;
extern crate chrono;
extern crate term_todo;


use diesel::PgConnection;
use diesel::prelude::*;
use self::term_todo::*;
use std::io::stdin;
use std::env::args;
use models::Task;

pub struct Database {
    pub conn: PgConnection
}

impl Database {
    pub fn new(conn: PgConnection) -> Database {
        Database {
            conn
        }
    }

    pub fn show_tasks(self) {
        use term_todo::schema::tasks::dsl::*;

        let conn = self.conn;
        let res: Vec<Task> = tasks.limit(5)
            .load::<Task>(&conn)
            .expect("Error loading tasks");

        println!("There are {} tasks", res.len());
        for task in res {
            println!("{}: {} ==> {}", task.id, task.title, task.in_progress);
        }
    }

    pub fn add_task(self) {
        let conn = self.conn;

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

    pub fn delete_task(self) {
        use schema::tasks::dsl::*;

        let target = args().nth(1).expect("Expected target title");
        let pattern = format!("%{}%", target);
        let conn = self.conn;

        let num_deleted = diesel::delete(tasks.filter(title.like(pattern)))
            .execute(&conn)
            .expect("Failed to delete!");
        println!("Deleted {} posts", num_deleted);
    }

    pub fn update_data(self, arg: bool) {
        use schema::tasks::dsl::*;
        let conn = self.conn;
        if arg {
            // Get command line arguments
            let target_id = args().nth(1).expect("publish_post requires post id")
                .parse::<i32>().expect("Invalid ID");


            // Update specified post
            let task: Task = diesel::update(tasks.find(target_id))
                .set(in_progress.eq(true))
                .get_result(&conn)
                .expect(&format!("Unable to find task {}", target_id));
            println!("Changed status of task {} to in progress", task.title);
        } else {
            println!("Target_id: ");
            let mut target_id = String::new();
            stdin().read_line(&mut target_id).unwrap();
            let target_id = target_id.trim_end().parse::<i32>().unwrap();

            // Update specified post
            let task: Task = diesel::update(tasks.find(target_id))
                .set(in_progress.eq(true))
                .get_result(&conn)
                .expect(&format!("Unable to find task {}", target_id));
            println!("Changed status of task {} to in progress", task.title);

        }
    }

}
