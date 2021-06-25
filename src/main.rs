extern crate ctrlc;
extern crate shlex;

mod commands;

use std::process::{Command, Stdio, Child};
use shlex::split;
use ctrlc::set_handler;
use std::io::{
    stdin,
    stdout,
    Write,
};

use commands::{
    cd,
    exit,
    dir,
    echo
};

fn main() {
    let error: String = String::from("An unexpected error happened. Please open an issue at https://github.com/KanseiKy/RustyShell");

    set_handler(move || {
        exit::exit();
    }).expect(&error);

    println!("  _____           _          _____ _          _ _");
    println!(" |  __ \\         | |        / ____| |        | | |");
    println!(" | |__) |   _ ___| |_ _   _| (___ | |__   ___| | |");
    println!(" |  _  / | | / __| __| | | |\\___ \\| '_ \\ / _ \\ | |");
    println!(" | | \\ \\ |_| \\__ \\ |_| |_| |____) | | | |  __/ | |");
    println!(" |_|  \\_\\__,_|___/\\__|\\__, |_____/|_| |_|\\___|_|_|");
    println!("                       __/ |");
    println!("                      |___/     verion: 0.1.0-Beta");

    loop {
        print!("$ ");
        stdout().flush().expect(&error);

        let mut input = String::new();
        stdin()
            .read_line(&mut input)
            .unwrap();

        
            let mut commands = input.trim().split(" | ").peekable();
            let mut prev = None;
    
            while let Some(command) = commands.next()  {
                let mut parts = split(command).unwrap().into_iter();
                let command = parts.next();

                if command == None {
                    continue;
                }

                let args = parts;
    
                match command.as_ref().clone().unwrap().as_str() {
                    "cd" => {
                        cd::cd(args);
                        prev = None;
                    },

                    "dir" => dir::dir(args),
                    "echo" => echo::echo(args),
                    "exit" => exit::exit(),
                    command => {
                        let stdin = prev
                            .map_or(
                                Stdio::inherit(),
                                |output: Child| Stdio::from(output.stdout.unwrap())
                            );
    
                        let stdout = if commands.peek().is_some() {
                            Stdio::piped()
                        } else {
                            Stdio::inherit()
                        };
    
                        let output = Command::new(command)
                            .args(args)
                            .stdin(stdin)
                            .stdout(stdout)
                            .spawn();
    
                        match output {
                            Ok(output) => { prev = Some(output); },
                            Err(_) => {
                                prev = None;
                                println!("RustyShell: {}: Command not found", command)
                            },
                        };
                    }
                }
            }
    
            if let Some(mut cmd) = prev {
                cmd.wait().unwrap();
            }
    }
}