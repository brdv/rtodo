use std::io;

use clap::Parser;
use rtodo::{initialise_if_needed, list, Cli, Commands};

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
        Commands::Add { title } => todo!(),
        Commands::Do { id } => todo!(),
    };

    Ok(())
}
