use std::io;

use clap::Parser;
use rtodo::commands::{add, do_todo, list, Commands};
use rtodo::utils::find_todo;
use rtodo::{initialise_if_needed, Cli};

fn main() -> io::Result<()> {
    initialise_if_needed()?;

    let rtodo = Cli::parse();

    println!("rtodo {} - a brdv product\n", env!("CARGO_PKG_VERSION"));

    match rtodo.command {
        Commands::List(args) => {
            for todo in list(&args) {
                println!("{}", todo);
            }
        }
        Commands::Add { title } => match add(title.as_str()) {
            Ok(added) => {
                println!(
                    "Added the following todo:\n\t{} \nat location: \n\t{}",
                    added, added.path
                )
            }
            Err(e) => {
                eprintln!("Something went wrong while creating the todo: {}", e)
            }
        },
        Commands::Do { id } => match do_todo(id) {
            Some(todo) => println!("Moved todo with id {} to {}", id, todo.path),
            None => eprintln!("Todo with id {} could not be found", id),
        },
        Commands::Path { id } => match find_todo(id) {
            Some(todo) => println!("{}", todo.path),
            None => eprintln!("Todo with id {} could not be found", id),
        },
    };

    Ok(())
}
