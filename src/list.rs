use std::fs::{self, DirEntry};

use crate::{get_rtodo_done_location, get_rtodo_todo_location, todo::Todo, ListArgs};

pub fn list(args: &ListArgs) -> Vec<Todo> {
    let mut todos = get_todos(args.done);
    if args.all {
        let dones = get_todos(!args.done);
        todos.extend(dones);
    }

    todos.sort_by_key(|todo| todo.id);
    todos
}

fn get_todos(done: bool) -> Vec<Todo> {
    let location = if done {
        get_rtodo_done_location()
    } else {
        get_rtodo_todo_location()
    };

    get_files_from(location.as_str())
        .into_iter()
        .filter_map(|file| parse_entry(&file))
        .collect()
}

fn get_files_from(location: &str) -> Vec<DirEntry> {
    let paths = fs::read_dir(location)
        .expect(format!("Given location doesnt exist on machine, {}", location).as_str());

    paths
        .filter_map(|entry| {
            if let Ok(path) = entry {
                return Some(path);
            }
            None
        })
        .collect()
}

fn parse_entry(entry: &DirEntry) -> Option<Todo> {
    if is_valid_todo_string(entry.file_name().to_str()?) {
        let path = entry.path();
        if let Some(path_str) = path.to_str() {
            return Some(Todo::from_path(path_str));
        } else {
            eprintln!("Could not convert path to string: {:?}", path);
        }
    }

    None
}

fn is_valid_todo_string(string: &str) -> bool {
    let split: Vec<&str> = string.split(".").collect();

    split.len() == 4
        // id is valid u32
        && split[0].trim().parse::<u32>().is_ok()
        // task is not empty
        && !split[1].trim().is_empty()
        // contains rtodo suffix
        && split[2].trim() == "rtodo"
        // is md file
        && split[3].trim() == "md"
}

#[cfg(test)]
mod tests {
    use crate::list::is_valid_todo_string;

    #[test]
    fn test_valid_todo_str() {
        let task = "0.task.rtodo.md";

        assert!(is_valid_todo_string(task));
    }

    #[test]
    fn test_no_rtodo_suffix() {
        let task = "0.task.todor.md";

        assert!(!is_valid_todo_string(task));
    }

    #[test]
    fn test_no_md_file() {
        let task = "0.task.rtodo";

        assert!(!is_valid_todo_string(task));
    }

    #[test]
    fn test_id_no_u32() {
        let task = "x.task..rtodo.md";

        assert!(!is_valid_todo_string(task));
    }

    #[test]
    fn test_too_short() {
        let task = "0.task.md";

        assert!(!is_valid_todo_string(task));
    }
}
