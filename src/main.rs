use std::env;
use std::fs;
use std::io::{self, Write};
mod command_list;
use command_list::command_setup;
use command_list::get_prompt;

fn main() {
    
    let stdin = io::stdin();
    let mut command_history: Vec<String> = Vec::new();
    let list_of_commands = command_setup();
    
    loop {
        // Use get_prompt() to display a shorter prompt.
        print!("~ {} $ ", get_prompt());
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
                "echo" | "exit" | "type" | "help" | "cd" | "pwd" => println!("{tail} is a shell builtin"),
                _ => println!("{tail}: not found"),
            },
            Some("help") => {
                match tail.as_str() {
                    s if s.is_empty() => {
                        for (cmd, desc) in &list_of_commands {
                            println!("{} - {}", cmd, desc);
                        }
                    },
                    s if list_of_commands.get(s).is_some() => {
                        if let Some(desc) = list_of_commands.get(s) {
                            println!("{} - {}", tail, desc);
                        }
                    },
                    _ => {
                        println!("{}: is not a valid command", tail);
                    },
                }
            },
            Some("cls") | Some("clear") => println!("\x1B[2J\x1B[1;1H"),
            Some("history") => {
                for (i, cmd) in command_history.iter().enumerate() {
                    println!("{}: {}", i + 1, cmd);
                }
            },
            Some("cd") => {
                match tail.as_str() {
                    s if s.is_empty() => {
                        match env::current_dir() {
                            Ok(current_path) => println!("{}", current_path.display()),
                            Err(e) => println!("Error retrieving current directory: {}", e),
                        }
                    },                    
                    s if s == ".." => {
                        match env::current_dir() {
                            Ok(current_path) => {
                                if let Some(parent) = current_path.parent() {
                                    if let Err(e) = env::set_current_dir(parent) {
                                        println!("cd: failed to change directory: {}", e);
                                    }
                                } else {
                                    println!("cd: no parent directory");
                                }
                            },
                            Err(e) => println!("cd: error getting current directory: {}", e),
                        }
                    },
                    _ => {
                        if let Err(e) = env::set_current_dir(&tail) {
                            println!("cd: {}: {}", &tail, e);
                        }
                    },
                }
            },
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
            Some("ls") => {
                let current_dir = match env::current_dir() {
                    Ok(dir) => dir,
                    Err(e) => {
                        println!("Error retrieving current directory: {}", e);
                        continue;
                    }
                };
                match fs::read_dir(&current_dir) {
                    Ok(entries) => {
                        for entry_result in entries {
                            match entry_result {
                                Ok(entry) => {
                                    let file_name = entry.file_name().to_string_lossy().to_string();
                                    match entry.metadata() {
                                        Ok(metadata) => {
                                            if metadata.is_dir() {
                                                println!("{}/", file_name);
                                            } else {
                                                println!("{}", file_name);
                                            }
                                        },
                                        Err(e) => println!("Error reading metadata: {}", e),
                                    }
                                },
                                Err(e) => println!("Error reading directory entry: {}", e),
                            }
                        }
                    },
                    Err(e) => println!("Error reading directory: {}", e),
                }
            },            
            _ => println!("{}: command not found", input.trim()),
        }
    }
}
