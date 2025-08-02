use std::io::Error;

#[derive(Debug)]
pub enum InitError {
    Io(String),
    Json(String),
}

impl From<Error> for InitError {
    fn from(e: std::io::Error) -> Self {
        InitError::Io(e.to_string())
    }
}

impl From<String> for InitError {
    fn from(s: String) -> Self {
        InitError::Json(s)
    }
}