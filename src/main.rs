use clap::Parser;
use rtodo::{initialise_if_needed, Cli, Commands};

fn main() {
    initialise_if_needed();

    let rtodo = Cli::parse();

    println!("rtodo {} - a brdv product\n", env!("CARGO_PKG_VERSION"));

    match rtodo.command {
        Commands::List(args) => todo!(),
        Commands::Add { title } => todo!(),
        Commands::Do { id } => todo!(),
    };
}
