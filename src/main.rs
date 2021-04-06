extern crate term_todo;

mod api;
mod help;

use self::term_todo::*;
use api::Database;
use help::help;
use std::env::args;

fn main() {
    let conn = establish_connection();
    let db = Database::new(conn);

    // Try to get CLI args or display all tasks
    let arg = args().nth(1);
    let arg = match arg {
        Some(val) => val,
        None => String::new(),
    };

    if arg.len() == 0 {
        db.display_board();
        return;
    }
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
        "done" => {
            let target_id = args().nth(2).unwrap_or(String::new());
            db.set_done(target_id);
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
        "board" => db.display_board(),
        "help" => help(),
        _ => {
            println!("Invalid argument")
        }
    }
}
