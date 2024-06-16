use crate::{todo::Todo, ListArgs};

pub fn list(args: &ListArgs) -> Vec<Todo> {
    let mut todos = Todo::get_todos(args.done);
    if args.all {
        let others = Todo::get_todos(!args.done);
        todos.extend(others);
    }

    todos.sort_by_key(|todo| todo.id);
    todos
}
