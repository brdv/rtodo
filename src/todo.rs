use std::fmt::Display;

use crate::{from_slug, get_rtodo_done_location};

#[derive(Debug)]
pub struct Todo {
    pub id: u32,
    pub task: String,
    pub status: TodoStatus,
    pub path: String,
}

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
