use crate::todo::Todo;

pub fn do_todo(id: u32) -> Option<Todo> {
    let item = Todo::get_todos_with_filter(false)
        .into_iter()
        .find(|todo| todo.id == id);

    item.map(|todo| todo.mark_done().expect("whoops"))
}
