use std::{
    fs::{File, create_dir},
    path::Path,
    vec,
};
use serde_json::to_writer_pretty;
use crate::models::migrations_project_configuration::MigrationsProjectConfiguration;

pub fn init_migrations_project(project_name: &str) -> Result<(), String> {
    let project_path = Path::new(&project_name);

    println!("Initializing the '{project_name}' repository");

    match create_migrations_project_directory(project_path) {
        Ok(_) => {}
        Err(_) => {
            return Err(format!(
                "Failed to create the '{project_name}' repository folder"
            ));
        }
    };

    match create_migrations_project_migrations_folder(project_path) {
        Ok(_) => {}
        Err(_) => {
            return Err(format!(
                "Failed to create the '{project_name}' repository migrations folder"
            ));
        }
    }

    match create_migrations_project_configuration_file(&project_name, project_path) {
        Ok(_) => {}
        Err(_) => {
            return Err(format!(
                "Failed to create the '{project_name}' repository configuration file"
            ));
        }
    }

    Ok(())
}

fn create_migrations_project_directory(project_path: &Path) -> std::io::Result<()> {
    create_dir(project_path)?;

    Ok(())
}

fn create_migrations_project_migrations_folder(project_path: &Path) -> std::io::Result<()> {
    let migrations_folder_path = Path::new(project_path).join("migrations");

    create_dir(migrations_folder_path)?;

    Ok(())
}

fn create_migrations_project_configuration_file(
    project_name: &str,
    project_path: &Path,
) -> std::io::Result<()> {
    let configuraton_file_path = Path::new(project_path).join("config.json");

    let created_file = File::create(&configuraton_file_path)?;

    fill_default_migrations_project_configuration_file(&project_name, &created_file)?;

    Ok(())
}

fn fill_default_migrations_project_configuration_file(
    project_name: &str,
    configuration_file: &File,
) -> std::io::Result<()> {
    let default_configuration_file = MigrationsProjectConfiguration {
        project_name: project_name.to_string(),
        connection_strings: vec![],
    };

    to_writer_pretty(configuration_file, &default_configuration_file)?;

    Ok(())
}
