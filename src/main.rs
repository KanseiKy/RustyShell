mod commands;

use std::process::Command;
use std::io::{
    stdout,
    stdin,
    Write,
};

use commands::{
    cd,
    exit
};

fn main() {
    let error = String::from("There was an error while reading input");

    loop {
        print!("$ ");
        stdout().flush().expect(&error);

        let mut input = String::new();
        stdin()
            .read_line(&mut input)
            .unwrap();

        let mut parts = input.trim().split_ascii_whitespace();
        let command = parts.next().unwrap();
        let args = parts;

        match command {
            "cd" => cd::cd(args),
            "exit" => return exit::exit(),

            _ => {
                let child = Command::new(command)
                    .args(args)
                    .spawn();

                match child {
                    Ok(mut child) => { child.wait().expect(&error); },
                    Err(e) => eprint!("{}", e)
                }
            }
        }
    }
}