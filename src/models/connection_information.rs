use serde::{Deserialize, Serialize};

use crate::models::suported_providers::SuportedProviders;

#[derive(Serialize, Deserialize, Debug)]
pub struct ConnectionInformation {
    pub id: String,
    pub user: String,
    pub password: String,
    pub provider: SuportedProviders,
}
