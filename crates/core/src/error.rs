use std::fmt;

#[derive(Debug)]
pub enum BootsError {
    AlreadyExists(String),
    IoError(std::io::Error),
    TemplateError(String),
    Other(String),
}

impl fmt::Display for BootsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BootsError::AlreadyExists(path) => write!(f, "Already exists: {}", path),
            BootsError::IoError(err) => write!(f, "IO error: {}", err),
            BootsError::TemplateError(err) => write!(f, "Template error: {}", err),
            BootsError::Other(err) => write!(f, "{}", err),
        }
    }
}
impl std::error::Error for BootsError {}

impl From<std::io::Error> for BootsError {
    fn from(err: std::io::Error) -> Self {
        BootsError::IoError(err)
    }
}
