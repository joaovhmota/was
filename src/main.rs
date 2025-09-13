use clap::Parser;

use crate::{
    cli::{Cli, execute_command},
    logging::logger::enable_verbose,
};

mod cli;
mod commands;
mod logging;
mod models;

fn main() {
    let cli = Cli::parse();

    if cli.verbose {
        enable_verbose();
    }

    execute_command(&cli.command);
}
