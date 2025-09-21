use clap::Parser;

use crate::{
    commands::{commands::Commands, init::init::init, new::new::new},
    logging::logger::error,
};

#[derive(Parser)]
#[command(author = "João Vinícius Hinkeldey Mota", version = "1.0.0", about = "Rust CLI tool to help you running your SQL scripts with two-way script execution. ", long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[arg(
        short,
        long,
        global = true,
        help = "Show what is happening behind the scenes"
    )]
    pub verbose: bool,

    #[command(subcommand)]
    pub command: Commands,
}

pub fn execute_command(command: &Commands) {
    println!();

    let command_result = match command {
        Commands::Init(init_args) => init(init_args),
        Commands::New(new_args) => new(new_args),
    };

    match command_result {
        Ok(_) => {}
        Err(error_message) => {
            error(&error_message);
        }
    }

    println!();
}
