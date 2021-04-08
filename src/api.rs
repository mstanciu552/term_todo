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
use std::io::stdin;

pub struct Database {
    conn: PgConnection,
}

impl Database {
    pub fn new(conn: PgConnection) -> Database {
        Database { conn }
    }
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
        let mut col_done: Vec<Task> = Vec::new();
        for task in res {
            match task.in_progress {
                Some(true) => col_in_progress.push(task),
                Some(false) => col_not_in_progress.push(task),
                None => col_done.push(task),
            }
        }

        // Display solution
        println!("{}", "=================================".red());
        println!("{}", "|To Do\t\t\t\t|".red().bold());
        println!("{}", "=================================\n".red());
        println!("---------------------------------");
        for task in &col_not_in_progress {
            println!(
                "|{}: {}|\n|In Progress: {}\t\t|\n|Created at: {}\t\t|\n|Due date: {}\t\t|",
                task.id.to_string().as_str().blue(),
                task.title.magenta().bold(),
                task.in_progress
                    .unwrap_or(false)
                    .to_string()
                    .as_str()
                    .cyan(),
                task.created_at.to_string().as_str().green(),
                if task.until_at != None {
                    task.until_at.unwrap().to_string().as_str().red()
                } else {
                    "None\t".green()
                }
            );
            // If the current task id is the last task id don't print delimiter
            if task.id != col_not_in_progress.last().unwrap().id {
                println!("---------------------------------");
            }
        }

        println!("{}", "---------------------------------\n".blue());
        println!("{}", "=================================".blue());
        println!("{}", "|In Progress\t\t\t|".blue().bold().blue());
        println!("{}", "=================================\n".blue());
        println!("---------------------------------");
        for task in col_in_progress {
            println!(
                "|{}: {}|\n|In Progress: {}\t\t|\n|Created at: {}\t\t|\n|Due date: {}\t\t|",
                task.id.to_string().as_str().blue(),
                task.title.magenta().bold(),
                task.in_progress
                    .unwrap_or(false)
                    .to_string()
                    .as_str()
                    .cyan(),
                task.created_at.to_string().as_str().green(),
                if task.until_at != None {
                    task.until_at.unwrap().to_string().as_str().red()
                } else {
                    "None\t".green()
                }
            );
            println!("---------------------------------");
        }
        println!("{}", "\n=================================".green());
        println!("{}", "|Done\t\t\t\t|".green().bold());
        println!("{}", "=================================\n".green());
        println!("---------------------------------");
        for task in col_done {
            println!(
                "|{}: {}|\n|In Progress: {}\t\t|\n|Created at: {}\t\t|\n|Due date: {}\t\t|",
                task.id.to_string().as_str().blue(),
                task.title.magenta().bold(),
                task.in_progress
                    .unwrap_or(false)
                    .to_string()
                    .as_str()
                    .cyan(),
                task.created_at.to_string().as_str().green(),
                if task.until_at != None {
                    task.until_at.unwrap().to_string().as_str().red()
                } else {
                    "None\t".green()
                }
            );
            // If the current task id is the last task id don't print delimiter
            if task.id != col_not_in_progress.last().unwrap().id {
                println!("---------------------------------");
            }
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

        println!(
            "There are {} tasks",
            res.len().to_string().as_str().yellow().bold()
        );
        for task in res {
            println!("---------------------------------");
            println!(
                "|{}: {}|\n|In Progress: {}\t\t|\n|Created at: {}\t\t|\n|Due date: {}\t\t|",
                task.id.to_string().as_str().blue(),
                task.title.magenta().bold(),
                task.in_progress
                    .unwrap_or(false)
                    .to_string()
                    .as_str()
                    .cyan(),
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
        println!("Saved task {}!", task.title.green().bold());
        self.show_tasks();
    }

    pub fn delete_task(&self, arg: String) {
        use schema::tasks::dsl::*;

        let conn = &self.conn;

        // Get task to delete based on argument
        if arg.len() > 0 {
            // Decide if the argument passed in is an ID or a TITLE
            let re = Regex::new(r"^\d*$").unwrap();
            let input_format = re.is_match(arg.as_str());

            if input_format {
                // Input is number based so an ID
                let target_id = arg.parse::<i32>().unwrap();

                let num_deleted = diesel::delete(tasks.find(target_id))
                    .execute(conn)
                    .expect("Failed to delete!");

                println!(
                    "Deleted {} posts",
                    num_deleted.to_string().as_str().yellow().bold()
                );
            } else {
                let num_deleted = diesel::delete(tasks)
                    .filter(title.eq(arg))
                    .execute(conn)
                    .expect("Failed to delete!");

                println!(
                    "Deleted {} posts",
                    num_deleted.to_string().as_str().yellow().bold()
                );
            }

        // Get task to delete based on input from keyboard
        } else {
            self.show_tasks();
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

                println!(
                    "Deleted {} posts",
                    num_deleted.to_string().as_str().yellow().bold()
                );
            }
        }
        self.show_tasks();
    }
    pub fn update_data(&self, arg: String) {
        use schema::tasks::dsl::*;
        self.show_tasks();

        // Get connection to db
        let conn = &self.conn;

        // Get task to change status based on argument
        if arg.len() > 0 {
            // TODO Add option for title based replace
            // Update specified post
            let task: Task = diesel::update(tasks.find(arg.parse::<i32>().unwrap()))
                .set(in_progress.eq(true))
                .get_result(conn)
                .expect(&format!("Unable to find task {}", arg));

            println!(
                "Changed status of task {} to {}",
                task.title.green().bold(),
                "in progress".blue().bold()
            );
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

            println!(
                "Changed status of task {} to {}",
                task.title.green().bold(),
                "in progress".blue().bold()
            );
        }
    }

    pub fn update_title(&self, arg: String) {
        use schema::tasks::dsl::*;
        self.show_tasks();

        // Get connection to db
        let conn = &self.conn;

        // Get task to change status based on argument
        if arg.len() > 0 {
            // Get new title
            println!("New Title: ");
            let mut new_title = String::new();
            stdin().read_line(&mut new_title).unwrap();
            let new_title = new_title.trim_end();

            // Update specified post
            let task: Task = diesel::update(tasks.find(arg.parse::<i32>().unwrap()))
                .set(title.eq(new_title))
                .get_result(conn)
                .expect(&format!("Unable to find task {}", arg));

            println!(
                "Changed title of task {} to {}",
                task.id.to_string().as_str().yellow().bold(),
                task.title.green().bold()
            );
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

            println!(
                "Changed title of task {} to {}",
                task.id.to_string().as_str().yellow().bold(),
                task.title.green().bold()
            );
        }
        self.show_tasks();
    }
    pub fn update_until(&self, arg: String) {
        use schema::tasks::dsl::*;
        self.show_tasks();

        // Get connection to db
        let conn = &self.conn;

        // Get task to change status based on argument
        if arg.len() > 0 {
            // Get new date
            println!("Until(YYYY-MM-DD || <empty>): ");
            let mut new_until_at = String::new();
            stdin().read_line(&mut new_until_at).unwrap();
            let new_until_at = new_until_at.trim_end();

            // If nothing is passed skip
            if new_until_at.len() == 0 {
                return;
            }

            // Convert string to NaiveDate
            let new_until_date =
                chrono::NaiveDate::parse_from_str(new_until_at, "%Y-%m-%d").unwrap();

            // Update specified post
            let task: Task = diesel::update(tasks.find(arg.parse::<i32>().unwrap()))
                .set(until_at.eq(new_until_date))
                .get_result(conn)
                .expect(&format!("Unable to find task {}", arg));

            println!(
                "Changed due date of task {} to {}",
                task.title.to_string().as_str().green().bold(),
                task.until_at.unwrap().to_string().as_str().red().bold()
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

            // If no date pased don't modify
            if new_until_at.len() == 0 {
                return;
            }

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
                task.title.green().bold(),
                task.until_at.unwrap().to_string().as_str().red().bold()
            );
        }
        self.show_tasks();
    }

    pub fn set_done(&self, target_id: String) {
        use schema::tasks::dsl::*;
        let conn = &self.conn;

        if target_id.len() == 0 {
            // Get keyboard input
            let mut tid = String::new();
            stdin().read_line(&mut tid).unwrap();
            let tid = tid.trim_end().parse::<i32>().unwrap();

            // Modify the status
            let task: Task = diesel::update(tasks.find(tid))
                .set(in_progress.eq(None as Option<bool>))
                .get_result(conn)
                .expect(&format!("Unable to find task {}", target_id));

            println!(
                "Updated task {} to status {}",
                task.title.to_string().as_str().magenta().bold(),
                "Done".green().bold()
            );

            return;
        }

        // Argument was passed in
        let task: Task = diesel::update(tasks.find(target_id.parse::<i32>().unwrap()))
            .set(in_progress.eq(None as Option<bool>))
            .get_result(conn)
            .expect(&format!("Unable to find task {}", target_id));

        println!(
            "Updated task {} to status {}",
            task.title.to_string().as_str().magenta().bold(),
            "Done".green().bold()
        );
    }
}
