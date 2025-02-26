use std::io::{self, Write};
use std::collections::HashMap;

fn command_setup() {
    let mut list_of_commands = HashMap::new();
    list_of_commands.insert("echo", "echo <text>");
    list_of_commands.insert("exit", "exit");
    list_of_commands.insert("type", "type <command>");
    list_of_commands.insert("help", "help (optional: <command>)");
    list_of_commands.insert("history", "history");
    list_of_commands.insert("cls", "cls");
    list_of_commands.insert("clear", "clear");
    return list_of_commands;
}
fn main() {
    let stdin = io::stdin();
    let mut command_history: Vec<String> = Vec::new();
    command_history();
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
    
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        command_history.push(input.trim().to_string());
        

        let mut command = input.split_whitespace();

        let head = command.next();
        let tail = command.collect::<Vec<&str>>().join(" ");
        
        match head {
            Some("exit") => std::process::exit(0),
            Some("echo") => println!("{}", tail),
            Some("type") => match tail.as_str() {
                "echo" | "exit" | "type" | "help" => println!("{tail} is a shell builtin"),
                _ => println!("{tail}: not found"),
            },
            Some("help") => {
                // for (cmd, desc) in &list_of_commands {
                //     println!("{} - {}", cmd, desc)
                // }
                match tail.as_str() {
                    s if s.is_empty() => {
                        for (cmd, desc) in &list_of_commands {
                            println!("{} - {}", cmd, desc)
                        }
                    },
                    s if list_of_commands.get(s).is_some() => {
                        match list_of_commands.get(s) {
                            Some(desc) => println!("{} - {}", tail, desc),
                            None => println!("{}: is not a valid command", tail)
                        }
                    },
                    _ => {
                        println!("{}: is not a valid command", tail)
                    },
                }
            },
            Some("cls") => println!("\x1B[2J\x1B[1;1H"),
            Some("clear") => println!("\x1B[2J\x1B[1;1H"),
            Some("history") => {
                for (i, cmd) in command_history.iter().enumerate() {
                    println!("{}: {}", i + 1, cmd)
                }
            },
            _ => println!("{}: command not found", input.trim())
        }
    }
}
