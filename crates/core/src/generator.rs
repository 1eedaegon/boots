use std::{fs, path::Path};

use crate::{BootsError, Result};

use dialoguer::Input;
use handlebars::Handlebars;
use include_dir::{Dir, include_dir};
use serde_json::{Value, json};

static TEMPLATE_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/../../template");

pub fn generate(name: Option<String>) -> Result<()> {
    let project_name = if let Some(name) = name {
        name
    } else {
        Input::<String>::new()
            .with_prompt("Project name")
            .default("my-project".to_string())
            .interact()
            .map_err(|e| BootsError::Other(format!("Failed to read input: {}", e)))?
    };
    let project_path = Path::new(&project_name);
    if project_path.exists() {
        return Err(BootsError::AlreadyExists(project_name));
    }
    let authors = get_git_author().unwrap_or_else(|| "author".to_string());
    process_template(&project_name, &authors)?;
    Ok(())
}

fn process_template(project_name: &str, authors: &str) -> Result<()> {
    let mut handlebars = Handlebars::new();
    handlebars.register_escape_fn(handlebars::no_escape);
    let context = json!({
        "project_name": project_name,
        "authors": authors
    });
    process_dir(&TEMPLATE_DIR, project_name, &handlebars, &context, "")?;
    Ok(())
}
fn process_dir(
    dir: &Dir,
    project_name: &str,
    handlebars: &Handlebars,
    context: &Value,
    current_path: &str,
) -> Result<()> {
    for entry in dir.entries() {
        match entry {
            include_dir::DirEntry::Dir(subdir) => {
                let subdir_name = subdir
                    .path()
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
                    .to_string();
                let dir_path = if current_path.is_empty() {
                    subdir_name
                } else {
                    format!("{}/{}", current_path, subdir_name)
                };
                process_dir(&subdir, project_name, handlebars, context, &dir_path)?;
            }
            include_dir::DirEntry::File(file) => {
                let file_path = if current_path.is_empty() {
                    file.path().to_string_lossy().to_string()
                } else {
                    format!(
                        "{}/{}",
                        current_path,
                        file.path().file_name().unwrap().to_string_lossy()
                    )
                };
                if file_path.contains("cargo-generate.toml") {
                    continue;
                }
                let content = file.contents_utf8().ok_or_else(|| {
                    BootsError::Other(format!("File is not UTF-8: {}", file_path))
                })?;
                let rendered_content = handlebars
                    .render_template(content, context)
                    .map_err(|e| BootsError::TemplateError(e.to_string()))?;

                let target_path = Path::new(project_name).join(&file_path);

                if let Some(parent) = target_path.parent() {
                    fs::create_dir_all(parent)?;
                }

                fs::write(&target_path, rendered_content)?;
            }
        }
    }
    Ok(())
}
fn get_git_author() -> Option<String> {
    std::process::Command::new("git")
        .args(["config", "user.name"])
        .output()
        .ok()
        .and_then(|output| {
            if output.status.success() {
                String::from_utf8(output.stdout)
                    .ok()
                    .map(|s| s.trim().to_string())
            } else {
                None
            }
        })
}
