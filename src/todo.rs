use std::{fmt::Display, fs::DirEntry};

use crate::{from_slug, get_files_from, get_rtodo_done_location, get_rtodo_todo_location};

#[derive(Debug)]
pub struct Todo {
    pub id: u32,
    pub task: String,
    pub status: TodoStatus,
    pub path: String,
}

// Static methods
impl Todo {
    pub fn from_path(path: &str) -> Self {
        let filename = path.split('/').collect::<Vec<&str>>().pop().unwrap();
        let parts = filename.split('.').collect::<Vec<&str>>();

        let done = path.starts_with(get_rtodo_done_location().as_str());

        Todo {
            id: parts[0].parse().unwrap_or(0),
            task: from_slug(parts[1]),
            status: if done {
                TodoStatus::Done
            } else {
                TodoStatus::Open
            },
            path: path.to_string(),
        }
    }

    // Retrieves todos either from the done or todo directory as specified in the configs
    pub fn get_todos(done: bool) -> Vec<Self> {
        let location = if done {
            get_rtodo_done_location()
        } else {
            get_rtodo_todo_location()
        };

        get_files_from(location.as_str())
            .into_iter()
            .filter_map(|file| Self::parse_entry(&file))
            .collect()
    }

    pub fn parse_entry(entry: &DirEntry) -> Option<Todo> {
        if Todo::is_valid_filename(entry.file_name().to_str()?) {
            let path = entry.path();
            if let Some(path_str) = path.to_str() {
                return Some(Todo::from_path(path_str));
            } else {
                eprintln!("Could not convert path to string: {:?}", path);
            }
        }

        None
    }

    pub fn is_valid_filename(string: &str) -> bool {
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
}

impl Display for Todo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}] {:0>3} {}",
            if self.status == TodoStatus::Done {
                "x"
            } else {
                " "
            },
            self.id,
            self.task
        )
    }
}

#[derive(Debug, PartialEq)]
pub enum TodoStatus {
    Done,
    Open,
}

#[cfg(test)]
mod tests {
    use crate::todo::Todo;

    #[test]
    fn test_valid_todo_str() {
        let task = "0.task.rtodo.md";

        assert!(Todo::is_valid_filename(task));
    }

    #[test]
    fn test_no_rtodo_suffix() {
        let task = "0.task.todor.md";

        assert!(!Todo::is_valid_filename(task));
    }

    #[test]
    fn test_no_md_file() {
        let task = "0.task.rtodo";

        assert!(!Todo::is_valid_filename(task));
    }

    #[test]
    fn test_id_no_u32() {
        let task = "x.task..rtodo.md";

        assert!(!Todo::is_valid_filename(task));
    }

    #[test]
    fn test_too_short() {
        let task = "0.task.md";

        assert!(!Todo::is_valid_filename(task));
    }
}
