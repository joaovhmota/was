use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MigrationsProjectConfiguration {
    pub project_name: String,
    pub connection_strings: Vec<ConnectionStringInformation>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConnectionStringInformation {
    pub name: String,
}
