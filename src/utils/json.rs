use std::{fs, io, path::Path};

// use serde::de::DeserializeOwned;
use serde_json::to_string_pretty;

use crate::models::project_configuration::ProjectConfiguration;

pub fn write_to_json<P: AsRef<Path>>(obj: &ProjectConfiguration, caminho: P) -> io::Result<()> {
    let json_str = to_string_pretty(obj).map_err(io::Error::other)?;

    fs::write(caminho, json_str)
}

// pub fn read_from_json<T: DeserializeOwned, P: AsRef<Path>>(caminho: P) -> io::Result<T> {
//     let json_string = fs::read_to_string(caminho)?;
//     let pessoa: T =
//         serde_json::from_str(&json_string).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

//     return Ok(pessoa);
// }
