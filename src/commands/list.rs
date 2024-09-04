use crate::todo::Todo;

use super::ListArgs;

pub fn list(args: &ListArgs) -> Vec<Todo> {
    let mut todos = if args.all {
        Todo::get_todos()
    } else {
        Todo::get_todos_with_filter(args.done)
    };

    todos.sort_by_key(|todo| todo.id);
    todos
}
