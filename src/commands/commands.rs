use clap::Subcommand;

use crate::commands::{init::init_args::InitArgs, new::new_args::NewArgs};

#[derive(Subcommand)]
pub enum Commands {
    Init(InitArgs),
    New(NewArgs),
}
