use crate::config::Config;
use crate::config::read_config;
use crate::prompt::run_prompt;

mod prompt;
mod config;
mod builtin;
mod child;
mod process;

fn main() {
    let config: Config = read_config();

    let mut input: String = String::new();
    loop {
        input.clear();

        if let Err(e) = run_prompt(&config, &mut input) {
            eprintln!("Error reading input: {}", e);
        } else {
            if let Some(e) = process::process(&input) {
                eprintln!("Error handling input: {}", e);
            }
        };
    };
}