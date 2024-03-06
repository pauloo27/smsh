use std::{env, process};

#[derive(Debug)]
pub struct AppOptions {
    pub definition_file: String,
    pub css_file: Option<String>,
}

impl AppOptions {
    pub fn new(definition_file: String, css_file: Option<String>) -> Self {
        AppOptions {
            definition_file,
            css_file,
        }
    }

    pub fn from_args() -> Self {
        let definition_file = match env::args().nth(1) {
            Some(v) => v,
            None => {
                eprintln!("Usage: smsh <definition file> [css file]");
                process::exit(1);
            }
        };
        let css_file = env::args().nth(2);

        AppOptions::new(definition_file, css_file)
    }
}
