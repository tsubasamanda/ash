use std::env;
use std::io::stderr;
use std::io::Write;
use std::path::Path;
use std::str::SplitWhitespace;

// Returns true if the requested command is a built-in program to be handled here
pub fn match_builtin(command: &str, args: &mut SplitWhitespace) -> bool {
    match command {
        "cd" => {
            let new_dir = args.peekable().peek().map_or("~", |x| *x);
            let root = Path::new(new_dir);
            if let Err(e) = env::set_current_dir(&root) {
                eprintln!("{}", e);
                stderr().flush().unwrap();
            }
        },
        "exit" => {
            std::process::exit(0);
        },
        _ => {
            return false;
        }
    }

    return true;
}