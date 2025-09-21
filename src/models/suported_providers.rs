use std::fmt::Display;

use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, ValueEnum)]
pub enum SuportedProviders {
    PostgreSQL,
    SQLServer,
}

impl Display for SuportedProviders {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SuportedProviders::PostgreSQL => write!(f, "PostgreSQL"),
            SuportedProviders::SQLServer => write!(f, "SQL Server"),
        }
    }
}
