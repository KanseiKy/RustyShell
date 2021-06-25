extern crate ctrlc;
mod commands;

// use std::process::Command;
use ctrlc::set_handler;
use std::io::{
    stdin,
    stdout,
    Write
};

use commands::{
    cd,
    exit,
    dir
};

fn main() {
    println!("  _____           _          _____ _          _ _");
    println!(" |  __ \\         | |        / ____| |        | | |");
    println!(" | |__) |   _ ___| |_ _   _| (___ | |__   ___| | |");
    println!(" |  _  / | | / __| __| | | |\\___ \\| '_ \\ / _ \\ | |");
    println!(" | | \\ \\ |_| \\__ \\ |_| |_| |____) | | | |  __/ | |");
    println!(" |_|  \\_\\__,_|___/\\__|\\__, |_____/|_| |_|\\___|_|_|");
    println!("                       __/ |");
    println!("                      |___/     verion: 0.1.0-Beta");

    let error: String = String::from("An unexpected error happened. Please open an issue at https://github.com/KanseiKy/RustyShell");

    set_handler(move || {
        return exit::exit();
    }).expect(&error);

    loop {
        print!("$ ");
        stdout().flush().expect(&error);

        let mut input = String::new();
        stdin()
            .read_line(&mut input)
            .unwrap();

        
        let mut commands = input.trim().split(" | ").peekable();
        // let mut prev = None;

        while let Some(commands) = commands.next() {
            let mut parts = commands.trim().split_whitespace();
            let command = parts.next();

            if command == None {
                return exit::exit();
            }

            let args = parts;

            match command.unwrap() {
                "dir" => dir::dir(args),
                "cd" => cd::cd(args),
                "exit" => return exit::exit(),
                _ => println!("RustyShell: {:?}: Command not found", command)
            }
        }
    }
}