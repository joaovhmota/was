use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum DatabaseType {
    #[serde(rename = "sqlite")]
    SQLite
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConnectionStringInformation {
    pub data_source: String,
    pub user: String,
    pub password: String,
    pub database_type: DatabaseType
}
