use std::path::PathBuf;

pub const MIGRATIONS_FOLDER_NAME: &str = "migrations";
pub const CONIGURATION_FILE_NAME: &str = "config.json";

pub fn get_relative_migrations_folder_path() -> PathBuf {
    PathBuf::new().join(MIGRATIONS_FOLDER_NAME)
}

pub fn get_relative_configuration_file_path() -> PathBuf {
    PathBuf::new().join(CONIGURATION_FILE_NAME)
}

pub fn is_in_migrations_project() -> bool {
    let migrations_file_path = get_relative_migrations_folder_path();
    let configuration_file_path = get_relative_configuration_file_path();

    migrations_file_path.exists() && configuration_file_path.exists()
}
