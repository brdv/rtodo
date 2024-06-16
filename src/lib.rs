mod add;
mod do_todo;
mod init;
mod list;
mod todo;
mod utils;

pub use add::add;
pub use do_todo::do_todo;
pub use init::initialise_if_needed;
pub use list::list;
pub use utils::*;

use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "rtodo")]
#[command(about, version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// List all available todos
    List(ListArgs),
    /// Add a todo to the list
    Add { title: String },
    /// Complete a todo
    Do { id: u32 },
}

#[derive(Args, Debug)]
pub struct ListArgs {
    #[arg(short, long)]
    /// show all todos
    all: bool,
    /// show only done todos
    #[arg(short, long)]
    done: bool,
}
