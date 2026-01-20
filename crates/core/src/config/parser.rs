use super::types::*;
use crate::error::{BootsError, Result};
use std::process::Command;

/// Read git config value, returning empty string if not found or on error
fn get_git_config(key: &str) -> String {
    Command::new("git")
        .args(["config", "--get", key])
        .output()
        .ok()
        .and_then(|output| {
            if output.status.success() {
                String::from_utf8(output.stdout).ok()
            } else {
                None
            }
        })
        .map(|s| s.trim().to_string())
        .unwrap_or_default()
}

pub fn parse_options(
    project_type: ProjectType,
    name: &str,
    options: Option<&str>,
) -> Result<ProjectConfig> {
    let author_name = get_git_config("user.name");
    let author_email = get_git_config("user.email");

    let mut config = ProjectConfig {
        name: name.to_string(),
        project_type,
        persistence: None,
        frontend: None,
        has_grpc: false,
        has_http: true,
        has_client: false,
        author_name,
        author_email,
    };

    if let Some(opts) = options {
        let opts_list: Vec<&str> = opts.split(',').map(|s| s.trim()).collect();

        // If 'sample' option is present, ignore all other options
        if opts_list.contains(&"sample") {
            let other_options: Vec<&str> = opts_list
                .iter()
                .filter(|o| **o != "sample")
                .copied()
                .collect();
            if !other_options.is_empty() {
                eprintln!(
                    "Warning: 'sample' option ignores other options: {:?}",
                    other_options
                );
            }
            // Sample project uses postgres and spa by default
            config.persistence = Some(PersistenceType::Postgres);
            config.frontend = Some(FrontendType::Spa);
            return Ok(config);
        }

        for option in opts_list {
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
                // Frontend options: fe:spa or fe:ssr
                "fe:spa" | "fe-spa" | "spa" => config.frontend = Some(FrontendType::Spa),
                "fe:ssr" | "fe-ssr" | "ssr" => config.frontend = Some(FrontendType::Ssr),
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
        let config =
            parse_options(ProjectType::Service, "test-svc", Some("postgres,grpc")).unwrap();
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

    #[test]
    fn test_parse_service_with_spa() {
        let config = parse_options(ProjectType::Service, "test-svc", Some("fe:spa")).unwrap();
        assert_eq!(config.frontend, Some(FrontendType::Spa));
    }

    #[test]
    fn test_parse_service_with_ssr() {
        let config = parse_options(ProjectType::Service, "test-svc", Some("fe:ssr")).unwrap();
        assert_eq!(config.frontend, Some(FrontendType::Ssr));
    }

    #[test]
    fn test_parse_service_with_postgres_and_spa() {
        let config =
            parse_options(ProjectType::Service, "test-svc", Some("postgres,fe:spa")).unwrap();
        assert_eq!(config.persistence, Some(PersistenceType::Postgres));
        assert_eq!(config.frontend, Some(FrontendType::Spa));
    }

    #[test]
    fn test_parse_sample_default() {
        let config = parse_options(ProjectType::Sample, "test-board", Some("sample")).unwrap();
        assert_eq!(config.project_type, ProjectType::Sample);
        assert_eq!(config.persistence, Some(PersistenceType::Postgres));
        assert_eq!(config.frontend, Some(FrontendType::Spa));
    }

    #[test]
    fn test_parse_sample_ignores_other_options() {
        // sample should ignore postgres,grpc and use default sample config
        let config = parse_options(
            ProjectType::Sample,
            "test-board",
            Some("sample,grpc,sqlite"),
        )
        .unwrap();
        assert_eq!(config.persistence, Some(PersistenceType::Postgres)); // not sqlite
        assert!(!config.has_grpc); // grpc ignored
        assert_eq!(config.frontend, Some(FrontendType::Spa));
    }
}
