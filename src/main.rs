use clap::Parser;

use crate::cli::{Cli, execute_command};

mod cli;
mod commands;
mod models;

fn main() {
    let cli = Cli::parse();

    execute_command(&cli.command);
}
