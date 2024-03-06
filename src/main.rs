mod def;
mod parser;
mod ui;

use std::process;

fn main() {
    let options = ui::AppOptions::from_args();
    process::exit(ui::run(options).into());
}
