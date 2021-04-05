extern crate chrono;
extern crate diesel;
extern crate regex;
extern crate term_todo;

use self::term_todo::*;
use colored::Colorize;
use diesel::prelude::*;
use diesel::PgConnection;
use models::Task;
use regex::Regex;
use std::env::args;
use std::io::stdin;

pub struct Database {
    conn: PgConnection,
}

impl Database {
    pub fn new(conn: PgConnection) -> Database {
        Database { conn }
    }
    // TODO Add command loop
    // Board display
    pub fn display_board(&self) {
        use term_todo::schema::tasks::dsl::*;

        let conn = &self.conn;
        let res: Vec<Task> = tasks
            .limit(20)
            .load::<Task>(conn)
            .expect("Error loading tasks");
        // Separate tasks to in progress and to do
        let mut col_not_in_progress: Vec<Task> = Vec::new();
        let mut col_in_progress: Vec<Task> = Vec::new();
        for task in res {
            match task.in_progress {
                true => col_in_progress.push(task),
                false => col_not_in_progress.push(task),
            }
        }

        // Display solution
        println!("=================================");
        println!("|To Do\t\t\t\t|");
        println!("=================================\n");
        println!("---------------------------------");
        for task in col_not_in_progress {
            println!(
                "|{}: {}\t\t\t|\n|In Progress: {}\t\t|\n|Created at: {}\t\t|\n|Due date: {}\t\t|",
                task.id.to_string().as_str().blue(),
                task.title.magenta().bold(),
                task.in_progress.to_string().as_str().cyan(),
                task.created_at.to_string().as_str().green(),
                if task.until_at != None {
                    task.until_at.unwrap().to_string().as_str().red()
                } else {
                    "None\t".green()
                }
            );
        }

        println!("---------------------------------\n");
        println!("=================================");
        println!("|In progress\t\t\t|");
        println!("=================================\n");
        println!("---------------------------------");
        for task in col_in_progress {
            println!(
                "|{}: {}\t\t\t|\n|In Progress: {}\t\t|\n|Created at: {}\t\t|\n|Due date: {}\t\t|",
                task.id.to_string().as_str().blue(),
                task.title.magenta().bold(),
                task.in_progress.to_string().as_str().cyan(),
                task.created_at.to_string().as_str().green(),
                if task.until_at != None {
                    task.until_at.unwrap().to_string().as_str().red()
                } else {
                    "None\t".green()
                }
            );
            println!("---------------------------------");
        }
    }
    // Display all tasks
    pub fn show_tasks(&self) {
        use term_todo::schema::tasks::dsl::*;

        let conn = &self.conn;
        let res: Vec<Task> = tasks
            .limit(5)
            .load::<Task>(conn)
            .expect("Error loading tasks");

        println!("There are {} tasks", res.len().to_string().as_str().bold());
        for task in res {
            println!("---------------------------------");
            println!(
                "|{}: {}\t\t\t|\n|In Progress: {}\t\t|\n|Created at: {}\t\t|\n|Due date: {}\t\t|",
                task.id.to_string().as_str().blue(),
                task.title.magenta().bold(),
                task.in_progress.to_string().as_str().cyan(),
                task.created_at.to_string().as_str().green(),
                if task.until_at != None {
                    task.until_at.unwrap().to_string().as_str().red()
                } else {
                    "None\t".green()
                }
            );
            println!("---------------------------------");
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
        self.show_tasks();
    }

    pub fn delete_task(&self, arg: bool) {
        use schema::tasks::dsl::*;
        let conn = &self.conn;
        self.show_tasks();

        // TODO Change delete to based on exact pattern
        // Get task to delete based on argument
        if arg {
            println!("Target_id: ");
            let target_id = args().nth(2).expect("Expected target title");
            // Decide if the argument passed in is an ID or a TITLE
            let re = Regex::new(r"^\d*$").unwrap();
            let input_format = re.is_match(target_id.as_str());
            if input_format {
                // Input is number based so an ID
                let target_id = target_id.parse::<i32>().unwrap();

                let num_deleted = diesel::delete(tasks.find(target_id))
                    .execute(conn)
                    .expect("Failed to delete!");
                println!("Deleted {} posts", num_deleted);
            } else {
                let num_deleted = diesel::delete(tasks)
                    .filter(title.eq(target_id))
                    .execute(conn)
                    .expect("Failed to delete!");
                println!("Deleted {} posts", num_deleted);
            }

        // Get task to delete based on input from keyboard
        } else {
            println!("Target id: ");
            let mut target = String::new();
            stdin().read_line(&mut target).unwrap();
            let target_id = target.trim_end();
            // Decide if the argument passed in is an ID or a TITLE
            let re = Regex::new(r"^\d*$").unwrap();
            let input_format = re.is_match(target_id);
            if input_format {
                // Input is number based so an ID
                let target_id = target_id.parse::<i32>().unwrap();

                let num_deleted = diesel::delete(tasks.find(target_id))
                    .execute(conn)
                    .expect("Failed to delete!");
                println!("Deleted {} posts", num_deleted);
            } else {
                let num_deleted = diesel::delete(tasks)
                    .filter(title.eq(target_id))
                    .execute(conn)
                    .expect("Failed to delete!");
                println!("Deleted {} posts", num_deleted);
            }
        }
        self.show_tasks();
    }
    pub fn update_data(&self, arg: bool) {
        use schema::tasks::dsl::*;
        self.show_tasks();
        // Get connection to db
        let conn = &self.conn;
        // Get task to change status based on argument
        if arg {
            // Get command line arguments
            let target_id: i32 = args()
                .nth(2)
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

    pub fn update_title(&self, arg: bool) {
        use schema::tasks::dsl::*;
        self.show_tasks();
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
            // Get new title
            println!("New Title: ");
            let mut new_title = String::new();
            stdin().read_line(&mut new_title).unwrap();
            let new_title = new_title.trim_end();

            // Update specified post
            let task: Task = diesel::update(tasks.find(target_id))
                .set(title.eq(new_title))
                .get_result(conn)
                .expect(&format!("Unable to find task {}", target_id));
            println!("Changed title of task {} to {}", task.id, task.title);
        // Get task to change status based on keyboard input
        } else {
            // Get keyboard input
            println!("Target_id: ");
            let mut target_id = String::new();
            stdin().read_line(&mut target_id).unwrap();
            let target_id = target_id.trim_end().parse::<i32>().unwrap();
            // Get new title
            println!("New Title: ");
            let mut new_title = String::new();
            stdin().read_line(&mut new_title).unwrap();
            let new_title = new_title.trim_end();

            // Update specified post
            let task: Task = diesel::update(tasks.find(target_id))
                .set(title.eq(new_title))
                .get_result(conn)
                .expect(&format!("Unable to find task {}", target_id));
            println!("Changed title of task {} to {}", task.id, task.title);
        }
        self.show_tasks();
    }
    pub fn update_until(&self, arg: bool) {
        use schema::tasks::dsl::*;
        self.show_tasks();
        // Get connection to db
        let conn = &self.conn;
        // Get task to change status based on argument
        if arg {
            // Get command line arguments
            let target_id = args()
                .nth(2)
                .expect("requires task id")
                .parse::<i32>()
                .expect("Invalid ID");
            // Get new date
            println!("Until(YYYY-MM-DD || <empty>): ");
            let mut new_until_at = String::new();
            stdin().read_line(&mut new_until_at).unwrap();
            let new_until_at = new_until_at.trim_end();
            // Convert string to NaiveDate
            let new_until_date =
                chrono::NaiveDate::parse_from_str(new_until_at, "%Y-%m-%d").unwrap();

            // Update specified post
            let task: Task = diesel::update(tasks.find(target_id))
                .set(until_at.eq(new_until_date))
                .get_result(conn)
                .expect(&format!("Unable to find task {}", target_id));
            println!(
                "Changed due date of task {} to {}",
                task.title,
                task.until_at.unwrap()
            );
            // Get task to change status based on keyboard input
        } else {
            // Get keyboard input
            println!("Target_id: ");
            let mut target_id = String::new();
            stdin().read_line(&mut target_id).unwrap();
            let target_id = target_id.trim_end().parse::<i32>().unwrap();
            // Get new date
            println!("Until(YYYY-MM-DD || <empty>): ");
            let mut new_until_at = String::new();
            stdin().read_line(&mut new_until_at).unwrap();
            let new_until_at = new_until_at.trim_end();

            // Convert string to NaiveDate
            let new_until_date =
                chrono::NaiveDate::parse_from_str(new_until_at, "%Y-%m-%d").unwrap();

            // Update specified post
            let task: Task = diesel::update(tasks.find(target_id))
                .set(until_at.eq(new_until_date))
                .get_result(conn)
                .expect(&format!("Unable to find task {}", target_id));
            println!(
                "Changed due date of task {} to {}",
                task.title,
                task.until_at.unwrap()
            );
        }
        self.show_tasks();
    }
}
