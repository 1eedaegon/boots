use super::types::*;
use crate::error::{BootsError, Result};

pub fn parse_options(project_type: ProjectType, name: &str, options: Option<&str>) -> Result<ProjectConfig> {
    let mut config = ProjectConfig {
        name: name.to_string(),
        project_type,
        persistence: None,
        has_grpc: false,
        has_http: true,
        has_client: false,
    };

    if let Some(opts) = options {
        for option in opts.split(',') {
            let option = option.trim();
            match option {
                "postgres" => config.persistence = Some(PersistenceType::Postgres),
                "sqlite" => config.persistence = Some(PersistenceType::Sqlite),
                "file" => config.persistence = Some(PersistenceType::File),
                "grpc" => config.has_grpc = true,
                "http" => config.has_http = true,
                "client" => config.has_client = true,
                "persistence" => {
                    if config.persistence.is_none() {
                        config.persistence = Some(PersistenceType::File);
                    }
                }
                "" => continue,
                _ => return Err(BootsError::InvalidOption(option.to_string())),
            }
        }
    }

    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_service_default() {
        let config = parse_options(ProjectType::Service, "test-svc", None).unwrap();
        assert_eq!(config.project_type, ProjectType::Service);
        assert!(config.persistence.is_none());
        assert!(!config.has_grpc);
        assert!(config.has_http);
    }

    #[test]
    fn test_parse_service_with_postgres() {
        let config = parse_options(ProjectType::Service, "test-svc", Some("postgres")).unwrap();
        assert_eq!(config.persistence, Some(PersistenceType::Postgres));
    }

    #[test]
    fn test_parse_service_with_multiple_options() {
        let config = parse_options(ProjectType::Service, "test-svc", Some("postgres,grpc")).unwrap();
        assert_eq!(config.persistence, Some(PersistenceType::Postgres));
        assert!(config.has_grpc);
    }

    #[test]
    fn test_parse_cli_with_client() {
        let config = parse_options(ProjectType::Cli, "test-cli", Some("client")).unwrap();
        assert!(config.has_client);
    }

    #[test]
    fn test_invalid_option() {
        let result = parse_options(ProjectType::Service, "test", Some("invalid_option"));
        assert!(result.is_err());
    }
}
