use std::{
    fs::{File, OpenOptions},
    io::{self, BufRead, BufReader, Read, Write},
};

use home;

pub const RTODO_ROOT_DIR: &str = ".rtodo";
pub const RTODO_TODO_DIR: &str = "todo";
pub const RTODO_DONE_DIR: &str = "done";

pub fn get_rtodo_todo_location() -> String {
    let mut path = get_rtodo_path();
    path.push_str("/");
    path.push_str(RTODO_TODO_DIR);

    path
}

pub fn get_rtodo_done_location() -> String {
    let mut path = get_rtodo_path();
    path.push_str("/");
    path.push_str(RTODO_DONE_DIR);

    path
}

pub fn get_rtodo_path() -> String {
    let home = home::home_dir().expect("Could not find HOME location");
    let home_str = home
        .to_str()
        .expect("Could not convert HOME path to string");

    let rtodo_path = format!("{}/{}", home_str, RTODO_ROOT_DIR);

    rtodo_path
}

pub fn get_config_location() -> String {
    let mut path = get_rtodo_path();
    path.push_str("/.config");

    path
}

pub fn get_last_id() -> u32 {
    let file = File::open(get_config_location()).expect("Could not open config file.");

    let mut reader = BufReader::new(file);

    // String to hold the first line
    let mut first_line = String::new();

    // Read the first line
    reader
        .read_line(&mut first_line)
        .expect("Could not read first line of config file.");

    let split: Vec<&str> = first_line.split('=').collect();

    if split.len() != 2 {
        panic!(
            "First line of config file should be id in format 'id=0', following line found: {}",
            first_line
        )
    }

    split[1]
        .parse::<u32>()
        .expect("Could not parse id in config file, please make sure it's a valid int.")
}

pub fn up_id() -> io::Result<()> {
    let id = get_last_id() + 1;
    write_id_to_config(id)?;

    Ok(())
}

pub fn write_id_to_config(id: u32) -> Result<(), io::Error> {
    let line = format!("id={}", id);
    write_first_line_of_file(&get_config_location(), line)?;
    Ok(())
}

fn write_first_line_of_file(path: &str, line: String) -> Result<(), io::Error> {
    let mut file_contents = Vec::new();
    {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        reader.read_to_end(&mut file_contents)?;
    }

    // Convert the contents to a string
    let file_contents_str = String::from_utf8_lossy(&file_contents);

    // Split the contents into lines
    let mut lines: Vec<&str> = file_contents_str.lines().collect();

    // Modify the first line
    if !lines.is_empty() {
        lines[0] = line.as_str();
    } else {
        // If the file is empty, add the new line
        lines.push(line.as_str());
    }

    // Join the lines back into a single string
    let new_contents = lines.join("\n");

    // Write the updated contents back to the file
    let mut file = OpenOptions::new().write(true).truncate(true).open(path)?;
    file.write_all(new_contents.as_bytes())?;

    Ok(())
}

pub fn create_slug(s: &str) -> String {
    let slug = s
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || c.is_ascii_whitespace())
        .collect::<String>()
        .replace(" ", "-")
        .to_lowercase();

    slug
}

pub fn from_slug(slug: &str) -> String {
    let replaced_spaces = slug.replace("-", " ");
    let original_string = replaced_spaces
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || c.is_ascii_whitespace())
        .collect::<String>();

    original_string
}
