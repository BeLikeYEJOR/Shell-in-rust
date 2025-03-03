use std::env;
use std::fs;
use chrono::Local;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;
use std::process::Command;
use std::thread;
use std::time::Duration;
// use std::net::TcpListener;
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

pub fn cat(file_name: String) {
    let file_text = match fs::read_to_string(file_name) {
        Ok(text) => text,
        Err(e) => {
            println!("{}", e);
            return;
        }

    };

    // let cleaned_text = file_text.replace("\n", "").replace("\t", "");
    println!("{}", file_text)
}

pub fn write(file_name: &str, text_content: &[&str]) {
    // let text = fs::write(file_name, text_content);
    if let Err(e) = fs::write(file_name, text_content.join(" ")) {
        println!("{}", e);
    }
}

pub fn writeon(file_name: &str, text_content: &[&str]) {
// Join the text slice into a single string separated by spaces.
    let text_to_append = text_content.join(" ");
    
    // Open the file in append mode, create it if it doesn't exist.
    let mut file = match OpenOptions::new().append(true).create(true).open(file_name) {
        Ok(f) => f,
        Err(e) => {
            println!("Error opening {}: {}", file_name, e);
            return;
        }
    };

    // Write the text to the file.
    if let Err(e) = file.write_all(text_to_append.as_bytes()) {
        println!("Error writing to {}: {}", file_name, e);
    } else {
        println!("Successfully appended to {}", file_name);
    }
}

pub fn wc(file_name: &str) {
       // Check if a file name was provided.
       if file_name.is_empty() {
        println!("wc: missing file name");
        return;
    }

    // Try to read the file as a string.
    match fs::read_to_string(file_name) {
        Ok(content) => {
            // Count the number of lines.
            let line_count = content.lines().count();
            // Count the number of words by splitting on whitespace.
            let word_count = content.split_whitespace().count();
            // Count the number of bytes.
            let byte_count = content.as_bytes().len();
            // Print the results.
            println!("{}: {} lines, {} words, {} bytes", file_name, line_count, word_count, byte_count);
        }
        Err(e) => println!("wc: error reading {}: {}", file_name, e),
    }
}

pub fn date() {
    let date = Local::now();
    println!("{:}", date);
}
