use std::fmt::Display;


#[derive(Debug)]
pub struct Error {
    pub message: String,
    pub source: Option<Box<dyn std::error::Error>>,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for Error {}
