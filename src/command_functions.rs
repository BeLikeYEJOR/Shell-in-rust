use std::env;
use std::fs;
use std::collections::HashMap;
// mod command_list;
// use command_list::command_setup;
// use command_list::get_prompt;

pub fn ls() {
    let current_dir = match env::current_dir() {
        Ok(dir) => dir,
        Err(e) => {
            println!("Error retrieving current directory: {}", e);
            return;
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
}

pub fn cd(tail: String) {
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
}

pub fn help(tail: String, list_of_commands:&HashMap<&str, &str>) {
    match tail.as_str() {
        s if s.is_empty() => {
            for (cmd, desc) in list_of_commands {
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
}

pub fn mv(source: &str, destination: &str) {
    match fs::rename(source, destination) {
        Ok(_) => println!("Moved {} to {}", source, destination),
        Err(e) => println!("Error moving {}: {}", source, e),
    }
}