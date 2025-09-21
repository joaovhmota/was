use serde::{Deserialize, Serialize};

use crate::models::connection_information::ConnectionInformation;

#[derive(Serialize, Deserialize, Debug)]
pub struct MigrationsProjectConfiguration {
    pub project_name: String,
    pub connection_strings: Vec<ConnectionInformation>,
}
