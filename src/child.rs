use std::io::Error;
use std::process::Child;
use std::process::Command;
use std::process::Stdio;
use std::str::SplitWhitespace;

pub fn spawn_child(command: &str, args: SplitWhitespace<'_>, stdout: Option<Stdio>, stdin: Option<Stdio>) -> Result<Child,Error> {
    return Command::new(command)
        .args(args)
        .stdout( stdout.unwrap_or(Stdio::inherit()))
        .stdin(stdin.unwrap_or(Stdio::inherit()))
        .stderr(Stdio::inherit())
        .spawn();
}