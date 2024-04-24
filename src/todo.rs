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
