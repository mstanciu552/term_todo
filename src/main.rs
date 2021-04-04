extern crate term_todo;

mod api;
mod help;

use self::term_todo::*;
use api::Database;
use help::help;
use std::env::args;

// TODO Change show list to have border based on text not fixed

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
                db.update_data(false);
            } else {
                db.update_data(true);
            }
        }
        "utitle" => {
            let target_id = args().nth(2);
            let cli_args = match target_id {
                Some(val) => val,
                None => String::from(""),
            };
            if cli_args.len() == 0 {
                db.update_title(false);
            } else {
                db.update_title(true);
            }
        }
        "udate" => {
            let target_id = args().nth(2);
            let cli_args = match target_id {
                Some(val) => val,
                None => String::from(""),
            };
            if cli_args.len() == 0 {
                db.update_until(false);
            } else {
                db.update_until(true);
            }
        }
        "help" => help(),
        _ => {
            println!("Invalid argument")
        }
    }
}
