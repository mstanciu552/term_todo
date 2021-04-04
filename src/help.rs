use colored::Colorize;

pub fn help() {
    println!("Usage:");
    println!("  ttask [OPTION] <PARAM>\n");
    println!("Options:");
    println!("  add\t\tAdd a new task");
    println!("  list\t\tShow current tasks");
    println!("  delete\tDelete specified(by {}) task", "id".blue());
    println!(
        "  doing\t\tUpdate the {} of a specified(by {}) task to {}",
        "status".green(),
        "id".blue(),
        "in progress".magenta().bold()
    );
    println!(
        "  utitle\t\tUpdate the {} of a specified(by {}) task",
        "title".magenta(),
        "id".blue()
    );
    println!(
        "  udate\t\tUpdate the {} of a specified(by {}) task",
        "due date".magenta(),
        "id".blue()
    );
}
