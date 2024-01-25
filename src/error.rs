use std::fmt::Display;


#[derive(Debug)]
pub struct Error {
    pub(crate) message: String,
    pub(crate) source: Option<std::io::Error>,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self
            .source
            .as_ref()
            .map(|error| error as &(dyn std::error::Error + 'static))
    }
}
