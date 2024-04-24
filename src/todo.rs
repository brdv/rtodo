pub const RTODO_ROOT_DIR: &str = ".rtodo";
pub const RTODO_TODO_DIR: &str = "todo";
pub const RTODO_DONE_DIR: &str = "done";

pub struct Todo {
    pub task: String,
    pub status: TodoStatus,
}

impl Todo {
    fn new(title: &str) -> Self {
        Todo {
            task: title.to_string(),
            status: TodoStatus::Open,
        }
    }
}

pub enum TodoStatus {
    Done,
    Open,
}
