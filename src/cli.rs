use clap::Parser;

use crate::commands::{Commands, init::init_migrations_project::init_migrations_project};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[arg(short, long, global = true)]
    pub verbose: bool,

    #[command(subcommand)]
    pub command: Commands,
}

pub fn execute_command(command: &Commands) {
    let _ = match command {
        Commands::Init(init_args) => init_migrations_project(&init_args.name),
    };
}
