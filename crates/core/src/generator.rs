use anyhow::Result;
use cargo_generate::{GenerateArgs, TemplatePath, generate as cargo_generator};

const TEMPLATE_REPO: &str = "https://github.com/1eedaegon/boots-template.git";

pub fn generate(name: Option<String>) -> Result<()> {
    // If name is provided, set it in define to skip the prompt
    let define = if let Some(ref n) = name {
        vec![format!("project_name={}", n)]
    } else {
        vec![]
    };

    let args = GenerateArgs {
        template_path: TemplatePath {
            git: Some(TEMPLATE_REPO.to_string()),
            branch: Some("main".to_string()),
            ..Default::default()
        },
        name,
        define,
        ..Default::default()
    };

    cargo_generator(args)?;
    Ok(())
}
