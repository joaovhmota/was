use clap::Parser;
use commands::AvaliableCommands;
use commands::call_command;

use crate::utils::logger::Logger;

mod commands;
mod models;
mod utils;

#[derive(Parser)]
#[command(name = "Write and Structure", version)]
pub struct Args {
    #[command(subcommand)]
    command: AvaliableCommands,

    #[arg(short = 'v', long = "verbose", global = true)]
    verbose: bool,
}

fn main() {
    let cli = Args::parse();

    if cli.verbose {
        Logger::verbose_mode();
    }

    call_command(&cli.command);
}
