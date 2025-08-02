use serde::{Deserialize, Serialize};

use crate::models::connection_string_information::ConnectionStringInformation;

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectConfiguration {
    pub name: String,
    pub connection_strings: Vec<ConnectionStringInformation>
}