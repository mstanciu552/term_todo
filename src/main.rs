extern crate term_todo;

mod api;

use self::term_todo::*;
use api::Database;
use std::env::args;

fn main() {
    let conn = establish_connection();
    let db = Database::new(conn);

    // CLI commands handling
    let arg = args().nth(1).expect("Getting arguments failed");

    match arg.as_str() {
        "add" => db.add_task(),
        "list" => db.show_tasks(),
        "delete" => {
            let target_id = args().nth(2);
            let cli_args = match target_id {
                Some(val) => val,
                None => String::new(),
            };

            if cli_args.len() == 0 {
                db.delete_task(false);
            } else {
                db.delete_task(true);
            }
        }
        "doing" => {
            // Either wait for another argument or ask for input
            let target_id = args().nth(2);
            let cli_args = match target_id {
                Some(val) => val,
                None => String::from(""),
            };
            if cli_args.len() == 0 {
                db.show_tasks();
                db.update_data(false);
            } else {
                db.show_tasks();
                db.update_data(true);
            }
        }
        _ => {
            println!("Invalid argument")
        }
    }
}
