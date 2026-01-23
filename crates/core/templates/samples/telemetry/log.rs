//! Structured logging with tracing-subscriber.

/// Log level configuration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl From<LogLevel> for tracing::Level {
    fn from(level: LogLevel) -> Self {
        match level {
            LogLevel::Trace => tracing::Level::TRACE,
            LogLevel::Debug => tracing::Level::DEBUG,
            LogLevel::Info => tracing::Level::INFO,
            LogLevel::Warn => tracing::Level::WARN,
            LogLevel::Error => tracing::Level::ERROR,
        }
    }
}

impl std::str::FromStr for LogLevel {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "trace" => Ok(LogLevel::Trace),
            "debug" => Ok(LogLevel::Debug),
            "info" => Ok(LogLevel::Info),
            "warn" | "warning" => Ok(LogLevel::Warn),
            "error" => Ok(LogLevel::Error),
            _ => Err(format!("Unknown log level: {}", s)),
        }
    }
}

/// Logging configuration
pub struct LogConfig {
    /// Log level
    pub level: LogLevel,
    /// Whether to output JSON format
    pub json: bool,
    /// Whether to include file/line info
    pub with_file: bool,
    /// Whether to include target
    pub with_target: bool,
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            level: LogLevel::Info,
            json: false,
            with_file: false,
            with_target: true,
        }
    }
}

impl LogConfig {
    /// Create config from environment
    pub fn from_env() -> Self {
        let level = std::env::var("RUST_LOG")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(LogLevel::Info);

        let json = std::env::var("LOG_FORMAT")
            .map(|s| s.to_lowercase() == "json")
            .unwrap_or(false);

        Self {
            level,
            json,
            ..Default::default()
        }
    }
}

/// Initialize logging with the given configuration
///
/// # Example
///
/// ```rust,ignore
/// use {{project_name_snake}}_core::telemetry::log::{init_logging, LogConfig};
///
/// // Use default configuration
/// init_logging()?;
///
/// // Or customize
/// let config = LogConfig {
///     level: LogLevel::Debug,
///     json: true,
///     ..Default::default()
/// };
/// init_with_config(config)?;
/// ```
pub fn init_logging() -> Result<(), Box<dyn std::error::Error>> {
    init_with_config(LogConfig::from_env())
}

/// Initialize logging with custom configuration
pub fn init_with_config(config: LogConfig) -> Result<(), Box<dyn std::error::Error>> {
    use tracing_subscriber::{fmt, prelude::*, EnvFilter};

    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(format!("{:?}", config.level).to_lowercase()));

    let subscriber = tracing_subscriber::registry().with(filter);

    if config.json {
        subscriber
            .with(
                fmt::layer()
                    .json()
                    .with_file(config.with_file)
                    .with_target(config.with_target),
            )
            .try_init()?;
    } else {
        subscriber
            .with(
                fmt::layer()
                    .with_file(config.with_file)
                    .with_target(config.with_target),
            )
            .try_init()?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_level_from_str() {
        assert_eq!("info".parse::<LogLevel>().unwrap(), LogLevel::Info);
        assert_eq!("DEBUG".parse::<LogLevel>().unwrap(), LogLevel::Debug);
        assert_eq!("Warning".parse::<LogLevel>().unwrap(), LogLevel::Warn);
    }

    #[test]
    fn test_log_config_default() {
        let config = LogConfig::default();
        assert_eq!(config.level, LogLevel::Info);
        assert!(!config.json);
    }
}
