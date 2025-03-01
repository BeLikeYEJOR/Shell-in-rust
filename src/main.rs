// use std::env;
use std::fs;
use std::io::{self, Write};
mod command_list;
mod command_functions;
use command_functions::ls;
use command_functions::cd;
use command_functions::help;
use command_list::command_setup;
use command_list::get_prompt;

fn main() {
    
    let stdin = io::stdin();
    let mut command_history: Vec<String> = Vec::new();
    let list_of_commands = command_setup();
    
    loop {
        // Use get_prompt() to display a shorter prompt.
        print!("{} $ ", get_prompt());
        io::stdout().flush().unwrap();

        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        let trimmed_input = input.trim().to_string();
        command_history.push(trimmed_input.clone());

        let mut command = input.split_whitespace();
        let head = command.next();
        let tail = command.collect::<Vec<&str>>().join(" ");

        match head {
            Some("exit") => std::process::exit(0),
            Some("echo") => println!("{}", tail),
            Some("type") => match tail.as_str() {
                "echo" | "exit" | "type" | "help" | "cd"  => println!("{tail} is a shell builtin"),
                _ => println!("{tail}: not found"),
            },
            Some("help") => {
                help(tail, &list_of_commands);
            },
            Some("cls") | Some("clear") => println!("\x1B[2J\x1B[1;1H"),
            Some("history") => {
                for (i, cmd) in command_history.iter().enumerate() {
                    println!("{}: {}", i + 1, cmd);
                }
            },
            Some("cd") => cd(tail),
            Some("mkdir") => {
                if let Err(e) = fs::create_dir_all(tail) {
                    println!("{}", e);
                }
            },
            Some("rmdir") => {
                if let Err(e) = fs::remove_dir(tail) {
                    println!("{}", e);
                }
            },
            Some("touch") => {
                if let Err(e) = fs::File::create(tail) {
                    println!("{}", e);
                }
            },
            Some("rmf") => {
                if let Err(e) = fs::remove_file(tail) {
                    println!("{}", e);
                }
            },
            Some("ls") => ls(),   
            _ => println!("{}: command not found", input.trim()),
        }
    }
}
