use clap::{Args, Subcommand};

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// List all available todos
    List(ListArgs),
    /// Add a todo to the list
    Add { title: String },
    /// Complete a todo
    Do { id: u32 },
    /// Get path for a todo file
    Path { id: u32 },
}

#[derive(Args, Debug)]
pub struct ListArgs {
    #[arg(short, long)]
    /// show all todos
    pub all: bool,
    /// show only done todos
    #[arg(short, long)]
    pub done: bool,
}
