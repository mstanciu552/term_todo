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
    let arg = args().nth(1).unwrap_or(String::new());
    if arg.len() == 0 {
        db.display_board();
        return;
    }
    match arg.as_str() {
        "add" => db.add_task(),
        "list" => db.show_tasks(),
        "delete" => {
            let target_id = args().nth(2).unwrap_or(String::new());
            db.delete_task(target_id);
        }
        "doing" => {
            // Either wait for another argument or ask for input
            let target_id = args().nth(2).unwrap_or(String::new());
            db.update_data(target_id);
        }
        "done" => {
            let target_id = args().nth(2).unwrap_or(String::new());
            db.set_done(target_id);
        }
        "utitle" => {
            let target_id = args().nth(2).unwrap_or(String::new());
            db.update_title(target_id);
        }
        "udate" => {
            let target_id = args().nth(2).unwrap_or(String::new());
            db.update_until(target_id);
        }
        "board" => db.display_board(),
        "help" => help(),
        _ => {
            println!("Invalid argument")
        }
    }
}
