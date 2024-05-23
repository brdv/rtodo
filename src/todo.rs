use std::fmt::Display;

use crate::get_rtodo_done_location;

#[derive(Debug)]
pub struct Todo {
    pub id: u32,
    pub task: String,
    pub status: TodoStatus,
}

impl Todo {
    pub fn from_path(path: &str) -> Self {
        let filename = path.split('/').collect::<Vec<&str>>().pop().unwrap();
        let parts = filename.split('.').collect::<Vec<&str>>();

        let done = path.starts_with(get_rtodo_done_location().as_str());

        Todo {
            id: parts[0].parse().unwrap_or(0),
            task: parts[1].to_string(),
            status: if done {
                TodoStatus::Done
            } else {
                TodoStatus::Open
            },
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
