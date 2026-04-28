use crate::prettyprint::constants::*;
use std::env;

pub struct PrettyPrinter {
    binary_name: Option<String>,
}

impl PrettyPrinter {
    pub fn new() -> Self {
        Self {
            binary_name: env::current_exe().ok().map(|p| {
                p.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("")
                    .to_string()
            }),
        }
    }

    pub fn print_version(&self) {
        println!("version {}", VERSION);
    }

    pub fn print_prefix(&self) {
        print!("{}: ", self.binary_name.as_deref().unwrap_or(PROJECT_NAME));
    }

    pub fn print_help_short(&self) {
        println!("Try {}", HELP);
    }

    pub fn print_help(&self) {
        println!("Usage: {}", USAGE);
        println!("{}", HELP_TEXT);
    }
}
