use clap::Parser;
use rtodo::{Cli, Commands};

fn main() {
    let rtodo = Cli::parse();

    println!("rtodo {} - a brdv product\n", env!("CARGO_PKG_VERSION"));

    match rtodo.command {
        Commands::List(args) => todo!(),
        Commands::Add { title } => todo!(),
        Commands::Do { id } => todo!(),
    };
}
