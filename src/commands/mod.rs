use clap::Subcommand;

use crate::commands::init::init_args::InitArgs;

pub mod init;

#[derive(Subcommand)]
pub enum Commands {
    Init(InitArgs),
}
