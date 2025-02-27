use std::env;
use std::path::PathBuf;
use std::collections::HashMap;

pub fn command_setup() -> HashMap<&'static str, &'static str> {
    let mut list_of_commands = HashMap::new();
    list_of_commands.insert("echo", "echo <text>");
    list_of_commands.insert("exit", "exit");
    list_of_commands.insert("type", "type <command>");
    list_of_commands.insert("help", "help (optional: <command>)");
    list_of_commands.insert("history", "history");
    list_of_commands.insert("cls", "cls");
    list_of_commands.insert("clear", "clear");
    list_of_commands.insert("cd", "cd (optional: <path>)");
    list_of_commands.insert("mkdir", "mkdir <dirName>");
    list_of_commands.insert("rmdir", "rmdir <dirName>");
    list_of_commands.insert("touch", "touch <fileName>");
    list_of_commands.insert("rmf", "rmf <fileName>");
    list_of_commands.insert("ls", "ls");
    list_of_commands
}

/// Returns a short prompt string:
/// - If inside the home directory, shows "~" (if exactly at home)
///   or "~\<last_component>" if in a subdirectory.
/// - Otherwise, shows the last component of the current directory.
pub fn get_prompt() -> String {
    // Get the current working directory.
    let current_dir = env::current_dir().unwrap_or_else(|_| PathBuf::from("unknown"));
    
    // Get the home directory (for Unix-like systems, "HOME"; on Windows, you might use "USERPROFILE")
    let home = env::var("HOME").unwrap_or_default();
    let home_path = PathBuf::from(&home);

    // If we have a home directory and the current directory is inside it,
    // try to show only the last part of the relative path.
    if !home.is_empty() {
        if let Ok(relative) = current_dir.strip_prefix(&home_path) {
            // If we're exactly in the home directory, show "~"
            if relative.as_os_str().is_empty() {
                return "~".to_string();
            } else {
                // Otherwise, display "~\<last_component>"
                if let Some(last) = relative.components().last() {
                    return format!("~{}{}", std::path::MAIN_SEPARATOR, last.as_os_str().to_string_lossy());
                }
            }
        }
    }
    // Fallback: show the last component of the full current directory.
    if let Some(last) = current_dir.file_name() {
        return last.to_string_lossy().into_owned();
    }
    // If all else fails, display the full path.
    current_dir.to_string_lossy().into_owned()
}