use std::io::Error;
use std::io::stdin;
use std::io::stdout;
use std::io::Write;

use crate::config::Config;

pub fn create_prompt(_config: &Config) -> String {
    return "> ".to_string();
}

pub fn run_prompt(config: &Config, input: &mut String) -> Result<usize,Error> {
    print!("{}", create_prompt(config));
    stdout().flush()?;
    return stdin().read_line(input);
}