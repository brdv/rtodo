use crate::todo::Todo;

pub fn do_todo(id: u32) -> Option<Todo> {
    let item = Todo::get_todos(false)
        .into_iter()
        .find(|todo| todo.id == id);

    match item {
        Some(todo) => Some(todo.mark_done().expect("whoops")),
        None => None,
    }
}
