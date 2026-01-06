use thiserror::Error;

#[derive(Error, Debug)]
pub enum BootsError {
    #[error("Invalid project type: {0}")]
    InvalidProjectType(String),

    #[error("Invalid option: {0}")]
    InvalidOption(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Template error: {0}")]
    Template(String),

    #[error("Directory already exists: {0}")]
    DirectoryExists(String),
}

pub type Result<T> = std::result::Result<T, BootsError>;
