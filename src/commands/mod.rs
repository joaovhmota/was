use clap::Subcommand;
use colored::Colorize;

use crate::{
    commands::init::init_command::InitCommand, models::dispatchable_command::DispatchableCommand,
    utils::logger::Logger,
};

pub mod init;

#[derive(Subcommand, Debug)]
pub enum AvaliableCommands {
    Init(InitCommand),
}

pub fn call_command(cmd: &AvaliableCommands) {
    let commnd_result = match cmd {
        AvaliableCommands::Init(command) => command.execute(),
    };

    match commnd_result {
        Ok(result) => match result {
            Some(result_text) => Logger::ok(result_text.white().to_string()),
            None => println!(),
        },
        Err(error_message) => Logger::error(error_message.white().to_string()),
    }
}
