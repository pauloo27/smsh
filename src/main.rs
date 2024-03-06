mod def;
mod parser;
mod ui;

use std::process;

fn main() {
    let options = ui::AppOptions::from_args();
    ui::run(options);
    process::exit(1); // exit with error as default
}
