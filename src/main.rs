extern crate term_todo;

mod api;

use self::term_todo::*;
use api::Database;
use std::env::args;

// TODO Show tasks when deleting by keyboard input
// TODO Make update of task possible

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
        _ => {
            println!("Invalid argument")
        }
    }
}
