use clap::Parser;

use crate::{
    commands::{Commands, init::init_migrations_project::init_migrations_project},
    logging::logger::error,
};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
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
        Commands::Init(init_args) => init_migrations_project(&init_args.name, init_args.force),
    };

    match command_result {
        Ok(_) => {}
        Err(error_message) => {
            error(&error_message);
        }
    }

    println!();
}
