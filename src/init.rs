use crate::{
    get_config_location, write_id_to_config, RTODO_DONE_DIR, RTODO_ROOT_DIR, RTODO_TODO_DIR,
};
use home;
use std::{fs, io, path::Path};

pub fn initialise_if_needed() -> io::Result<()> {
    if !dirs_exist(get_dir_names()) || !config_exists() {
        initialise()?;
    }

    Ok(())
}

fn initialise() -> io::Result<()> {
    println!("Initialising rtodo...");
    create_dirs(get_dir_names());
    create_config_file()?;

    Ok(())
}

fn dirs_exist(locations: Vec<String>) -> bool {
    for location in locations {
        if !Path::new(&location).exists() {
            return false;
        }
    }

    true
}

fn config_exists() -> bool {
    Path::new(&get_config_location()).exists()
}

fn create_config_file() -> io::Result<()> {
    fs::File::create(get_config_location())
        .expect("An error occurred while creating the config file");

    write_id_to_config(0)?;
    Ok(())
}

fn create_dirs(locations: Vec<String>) {
    locations.iter().for_each(|location| {
        fs::create_dir(location).unwrap_or_else(|_| {
            println!("Skipping '{}' since it already exists", location);
        })
    });
}

fn get_dir_names() -> Vec<String> {
    let home = home::home_dir().expect("Could not find HOME location");

    let home_str = home
        .to_str()
        .expect("HOME location could not be cast to string");

    [
        format!("{}/{}", home_str, RTODO_ROOT_DIR),
        format!("{}/{}/{}", home_str, RTODO_ROOT_DIR, RTODO_DONE_DIR),
        format!("{}/{}/{}", home_str, RTODO_ROOT_DIR, RTODO_TODO_DIR),
    ]
    .into()
}
