use crate::{
    commands::init::init_args::InitArgs,
    logging::logger::{info, ok, warn},
    models::migrations_project_configuration::MigrationsProjectConfiguration,
};
use serde_json::to_writer_pretty;
use std::{
    fs::{File, create_dir, remove_dir_all},
    path::Path,
    vec,
};

pub fn init(args: &InitArgs) -> Result<Option<String>, String> {
    if args.force {
        warn("Forcing initialization of the project… hope you know what you are doing");
    }

    let project_name = &args.name;
    let project_path = Path::new(&project_name);

    if project_path.exists() {
        if !args.force {
            return Err("Migrations folder already exists".into());
        }

        info(format!("Removing existing '{project_name}' folder"));

        match remove_dir_all(project_path) {
            Ok(_) => {
                info(format!(
                    "Deleted existing '{project_name}' folder to create migrations project"
                ));
            }
            Err(error) => {
                return Err(error.to_string());
            }
        };
    }

    info(format!("Initializing '{project_name}' migrations project"));

    match create_migrations_project_directory(project_path) {
        Ok(_) => {
            info("Successfully created project folder.");
        }
        Err(error) => {
            return Err(error.to_string());
        }
    }

    match create_migrations_project_migrations_folder(project_path) {
        Ok(_) => {
            info("Successfully created migrations folder");
        }
        Err(_) => {
            return Err("Could not create migrations folder".into());
        }
    }

    match create_migrations_project_configuration_file(project_name, project_path) {
        Ok(_) => {}
        Err(_) => {
            return Err("Failed to create configuration file".into());
        }
    }

    ok(format!("Created '{project_name}' migrations project"));

    Ok(None)
}

fn create_migrations_project_directory(project_path: &Path) -> std::io::Result<()> {
    info("Creating project's folder");

    create_dir(project_path)?;

    Ok(())
}

fn create_migrations_project_migrations_folder(project_path: &Path) -> std::io::Result<()> {
    info("Creating project's migrations folder");

    let migrations_folder_path = Path::new(project_path).join("migrations");

    create_dir(migrations_folder_path)?;

    Ok(())
}

fn create_migrations_project_configuration_file(
    project_name: &str,
    project_path: &Path,
) -> std::io::Result<()> {
    info("Creating project's configuratio file");

    let configuraton_file_path = Path::new(project_path).join("config.json");

    let created_file = File::create(&configuraton_file_path)?;

    fill_default_migrations_project_configuration_file(project_name, &created_file)?;

    Ok(())
}

fn fill_default_migrations_project_configuration_file(
    project_name: &str,
    configuration_file: &File,
) -> std::io::Result<()> {
    info("Configuring default values for project's configuraton file");

    let default_configuration_file = MigrationsProjectConfiguration {
        project_name: project_name.to_string(),
        connection_strings: vec![],
    };

    to_writer_pretty(configuration_file, &default_configuration_file)?;

    Ok(())
}
