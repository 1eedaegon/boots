pub mod adder;
pub mod error;
pub mod generator;

pub use error::BootsError;
pub type Result<T> = std::result::Result<T, BootsError>;
