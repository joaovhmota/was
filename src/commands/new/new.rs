use std::fs::{File, create_dir};

use crate::{
    commands::{
        common::{
            paths::{get_relative_migrations_folder_path, is_in_migrations_project},
            time::migrations_time_string,
        },
        new::new_args::NewArgs,
    },
    logging::logger::{info, ok},
};

pub fn new(args: &NewArgs) -> Result<Option<String>, String> {
    if !is_in_migrations_project() {
        return Err("Not in a migrations project repository".to_string());
    }

    info("Creating new migrations");

    let migrations_name = &args.migrations_name;
    let creation_timestamp = migrations_time_string();
    let relative_migrations_folder_path = get_relative_migrations_folder_path();

    let new_migration_folder =
        relative_migrations_folder_path.join(format!("{creation_timestamp}-{migrations_name}"));
    let up_file_path =
        new_migration_folder.join(format!("{creation_timestamp}-{migrations_name}.up.sql"));
    let down_file_path =
        new_migration_folder.join(format!("{creation_timestamp}-{migrations_name}.down.sql"));

    match create_dir(new_migration_folder) {
        Ok(_) => {
            info(format!("Created '{migrations_name}' folder"));
        }
        Err(error) => {
            return Err(format!(
                "Could not create new '{migrations_name}' folder: {error}"
            ));
        }
    };

    match File::create(up_file_path) {
        Ok(_) => {
            info(format!("Created '{migrations_name}' up file"));
        }
        Err(error) => {
            return Err(format!(
                "Could not create new '{migrations_name}' up file: {error}"
            ));
        }
    }

    match File::create(down_file_path) {
        Ok(_) => {
            info(format!("Created '{migrations_name}' down file"));
        }
        Err(error) => {
            return Err(format!(
                "Could not create new '{migrations_name}' down file: {error}"
            ));
        }
    }

    ok(format!(
        "Succesfully created the new '{migrations_name}' migration"
    ));

    Ok(None)
}
