use std::{io::Error, process::Child};

use crate::{builtin::match_builtin, child::spawn_child};

pub fn process(input: &String) -> Option<Error> {
    return {
        let res = process_command(input);

        match res {
            Ok(_) => { None },
            Err(e) => {Some(e)}
        }
    };
}

fn process_command(command: &String) -> Result<Option<Child>,Error> {
    // Split by whitespace, and then pull first part as the command
    let mut args = command.trim().split_whitespace();
    let command = args.next().unwrap();

    if match_builtin(command, &mut args) {
        return Ok(None);
    }

    let res = spawn_child(command, args, None, None);
    match res {
        Ok(mut child) => {
            match child.wait() {
                Ok(_) => { return Ok(Some(child)); },
                Err(e) => {
                    return Err(e);
                }
            }
        },
        Err(e ) => {
            return Err(e);
        }
    }
}