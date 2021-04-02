extern crate term_todo;

mod api;

use self::term_todo::*;
use api::Database;

fn main() {
    let conn = establish_connection();
    let db = Database::new(conn);
    
    db.show_tasks();
    
}
