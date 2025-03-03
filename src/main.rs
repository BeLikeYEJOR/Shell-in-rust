// use std::env;
use std::fs::{self};
use std::io::{self, Write};
mod command_functions;
mod command_list;
use command_functions::cat;
use command_functions::cd;
use command_functions::date;
use command_functions::help;
use command_functions::ls;
use command_functions::mv;
use command_functions::ports;
use command_functions::run_port;
use command_functions::wc;
use command_functions::write;
use command_functions::writeon;
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
                "echo" | "exit" | "type" | "help" | "cd" => println!("{tail} is a shell builtin"),
                _ => println!("{tail}: not found"),
            },
            Some("help") => {
                help(tail, &list_of_commands);
            }
            Some("cls") | Some("clear") => println!("\x1B[2J\x1B[1;1H"),
            Some("history") => {
                for (i, cmd) in command_history.iter().enumerate() {
                    println!("{}: {}", i + 1, cmd);
                }
            }
            Some("cd") => cd(tail),
            Some("mkdir") => {
                if let Err(e) = fs::create_dir_all(tail) {
                    eprintln!("{}", e);
                }
            }
            Some("rmdir") => {
                if let Err(e) = fs::remove_dir(tail) {
                    eprintln!("{}", e);
                }
            }
            Some("touch") => {
                if let Err(e) = fs::File::create(tail) {
                    eprintln!("{}", e);
                }
            }
            Some("rmf") => {
                if let Err(e) = fs::remove_file(tail) {
                    eprintln!("{}", e);
                }
            }
            Some("mv") => {
                let args: Vec<&str> = tail.split_whitespace().collect();
                if args.len() < 2 {
                    eprintln!("mv: missing source or destination argument");
                } else {
                    mv(args[0], args[1]);
                }
            }
            Some("cat") => cat(tail),
            Some("ls") => ls(),
            Some("write") => {
                let args: Vec<&str> = tail.split_whitespace().collect();
                if args.len() < 2 {
                    println!("Missing File or Text Content");
                } else {
                    write(args[0], &args[1..]);
                }
            }
            Some("writeon") => {
                let args: Vec<&str> = tail.split_whitespace().collect();
                if args.len() < 2 {
                    println!("Missing file or text content for writeon");
                } else {
                    writeon(args[0], &args[1..]);
                }
            }
            Some("wc") => {
                wc(tail.as_str());
            }
            // Some("mode") => {
            //     match tail.trim() {
            //         s if s == "history" => {
            //             if let Err(e) = history_mode(&command_history) {
            //                 println!("Error in history mode: {}", e);
            //             }
            //         }
            //         _ => println!("unknown mode")
            //     }
            // },
            Some("date") => date(),
            Some("ports") => ports(),
            Some("runport") => run_port(tail),
            _ => println!("{}: command not found", input.trim()),
        }
    }
}
