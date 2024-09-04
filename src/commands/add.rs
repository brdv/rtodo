// add function
// Take a title (single string now, maybe vectors later) as title and create and save a todo from it.
use std::fs::{remove_file, File};
use std::io::{self, Result};

use crate::todo::Todo;
use crate::utils::{create_slug, get_last_id, get_rtodo_todo_location, up_id};

pub fn add(title: &str) -> Result<Todo> {
    let path = create_new_todo_path(title);

    // try to create the file, propagate error otherwise
    File::create(&path)?;

    // Attempt to update the ID, and handle any errors.
    if let Err(e) = up_id() {
        // If updating the ID fails, remove the created file.
        remove_file(&path)?;

        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to update ID: {}", e),
        ));
    }
    Ok(Todo::from_path(&path))
}

fn create_new_todo_path(title: &str) -> String {
    let base = get_rtodo_todo_location();
    let id = get_last_id();

    format!("{}/{}.{}.rtodo.md", base, id, create_slug(title))
}
