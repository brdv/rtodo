use std::{fs, path::Path};

use crate::{get_rtodo_done_location, get_rtodo_todo_location, todo::Todo, ListArgs};

pub fn list(args: &ListArgs) -> Vec<Todo> {
    if args.all {
        let mut todos = get_todos(false);
        let dones = get_todos(true);
        todos.extend(dones);
        return todos;
    }

    get_todos(args.done)
}

fn get_todos(done: bool) -> Vec<Todo> {
    let location = if done {
        get_rtodo_done_location()
    } else {
        get_rtodo_todo_location()
    };

    get_files_from(location.as_str())
        .into_iter()
        .filter_map(|file| parse_path_str(file.as_str()))
        .collect()
}

fn get_files_from(location: &str) -> Vec<String> {
    let location_path = Path::new(location);

    let mut file_names: Vec<String> = Vec::new();

    if location_path.exists() && location_path.is_dir() {
        if let Ok(entries) = fs::read_dir(location_path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Some(file_name) = entry.path().to_str() {
                        file_names.push(file_name.to_string());
                    }
                }
            }
        }
    }

    file_names
}

fn parse_path_str(path: &str) -> Option<Todo> {
    let file = Path::new(path).file_name().unwrap().to_str().unwrap();

    if is_valid_todo_string(file) {
        return Some(Todo::from_path(path));
    }
    None
}

fn is_valid_todo_string(string: &str) -> bool {
    let split: Vec<&str> = string.split(".").collect();

    if split.len() < 4 {
        return false;
    }

    if split[0].trim().parse::<u32>().is_err() {
        return false;
    }

    if split[1].trim().is_empty() {
        return false;
    }

    if split[2].trim() != "rtodo" {
        return false;
    }

    if split[3].trim() != "md" {
        return false;
    }

    true
}
