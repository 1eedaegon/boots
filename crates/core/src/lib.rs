pub mod config;
pub mod error;
pub mod generator;
pub mod template;

pub use config::{parse_options, Module, PersistenceType, ProjectConfig, ProjectType};
pub use error::{BootsError, Result};
pub use generator::ProjectGenerator;
pub use template::{TemplateEngine, Templates};
