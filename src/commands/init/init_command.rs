use std::fs;
use std::fs::OpenOptions;
use std::path::Path;

use crate::commands::init::init_error::InitError;
use crate::models::connection_string_information::{ConnectionStringInformation, DatabaseType};
use crate::models::dispatchable_command::DispatchableCommand;
use crate::models::project_configuration::ProjectConfiguration;
use crate::utils::json::write_to_json;
use crate::utils::logger::Logger;
use clap::Args;
use colored::Colorize;

const MIGRATIONS_DIR: &str = "migrations";
const CONFIG_FILE: &str = "config.json";

#[derive(Args, Debug)]
pub struct InitCommand {
    #[arg(value_name = "PROJECT_NAME")]
    pub project_name: String,
}

impl DispatchableCommand for InitCommand {
    fn execute(&self) -> Result<Option<String>, String> {
        let project_name = &self.project_name;

        Logger::info(format!(
            "{} '{}'",
            "Initializing the project",
            project_name.bright_cyan()
        ));

        Logger::process("Checking if already in a migrations project.".to_string());

        if already_in_migrations_project() {
            return Err("Already in a migrations project.".to_string());
        }

        if let Err(e) = create_migrations_project(project_name) {
            let msg = match e {
                InitError::Io(inner) => format!("I/O error: {inner}"),
                InitError::Json(inner) => format!("Configuration error: {inner}"),
            };
            return Err(msg);
        }

        Ok(Some(format!(
            "{} '{}'",
            "Finished initializing the project",
            project_name.bright_cyan()
        )))
    }
}

fn already_in_migrations_project() -> bool {
    let migrations_folder = Path::new(MIGRATIONS_DIR);
    let configuration_json_file = Path::new(CONFIG_FILE);

    migrations_folder.is_dir() && configuration_json_file.is_file()
}

fn create_migrations_project(project_name: &str) -> Result<(), InitError> {
    Logger::process("Creating migrations project...".to_string());

    create_migrations_folder()?;
    create_configuration_file()?;
    write_configuration_value(project_name)?;

    Logger::process("Finished creating the migrations project!".to_string());
    Ok(())
}

fn create_migrations_folder() -> Result<(), InitError> {
    Logger::process("Creating migrations folder...".to_string());

    let migrations_path = Path::new(MIGRATIONS_DIR);
    fs::create_dir_all(migrations_path).map_err(InitError::from)?;

    Logger::process("Successfully created the migrations folder!".to_string());
    Ok(())
}

fn create_configuration_file() -> Result<(), InitError> {
    Logger::process("Creating configuration file...".to_string());

    let configuration_path = Path::new(CONFIG_FILE);

    OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(configuration_path)
        .map_err(InitError::from)?;

    Logger::process("Successfully created the configuration file!".to_string());
    Ok(())
}

fn write_configuration_value(project_name: &str) -> Result<(), InitError> {
    let configuration_path = Path::new(CONFIG_FILE);
    let project_configuration = ProjectConfiguration {
        name: project_name.to_string(),
        connection_strings: vec![ConnectionStringInformation {
            data_source: String::from(""),
            user: String::from(""),
            password: String::from(""),
            database_type: DatabaseType::SQLite,
        }],
    };

    write_to_json(&project_configuration, configuration_path)
        .map_err(|e| InitError::Json(e.to_string()))?;

    Ok(())
}
