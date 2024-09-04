// complete re-exports.
pub mod commands;
pub mod todo;
pub mod utils;

// private modules brought into scope.
mod init;

// partial re-exports
pub use init::initialise_if_needed;

use clap::Parser;
use commands::*;

#[derive(Parser, Debug)]
#[command(name = "rtodo")]
#[command(about, version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}
