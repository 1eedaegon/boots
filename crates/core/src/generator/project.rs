use crate::config::{Module, PersistenceType, ProjectConfig, ProjectType};
use crate::error::{BootsError, Result};
use crate::template::{TemplateEngine, Templates};
use std::fs;
use std::path::Path;

pub struct ProjectGenerator {
    config: ProjectConfig,
    engine: TemplateEngine,
}

impl ProjectGenerator {
    pub fn new(config: ProjectConfig) -> Self {
        let mut engine = TemplateEngine::new();
        engine.set("project_name", &config.name);
        engine.set("project_name_snake", &config.name.replace('-', "_"));

        Self { config, engine }
    }

    pub fn generate(&self, base_path: &Path) -> Result<()> {
        let project_path = base_path.join(&self.config.name);

        if project_path.exists() {
            return Err(BootsError::DirectoryExists(self.config.name.clone()));
        }

        fs::create_dir_all(&project_path)?;

        self.create_workspace(&project_path)?;
        self.create_github_workflows(&project_path)?;
        self.create_docker(&project_path)?;
        self.create_makefile(&project_path)?;
        self.create_readme(&project_path)?;
        self.create_gitignore(&project_path)?;
        self.create_rust_toolchain(&project_path)?;

        if self.config.has_grpc {
            self.create_proto(&project_path)?;
        }

        if self.config.persistence.is_some() {
            self.create_env_example(&project_path)?;
        }

        for module in self.config.modules() {
            self.create_module(&project_path, &module)?;
        }

        Ok(())
    }

    fn create_workspace(&self, path: &Path) -> Result<()> {
        let template = Templates::get_template("base/Cargo.workspace.toml")
            .ok_or_else(|| BootsError::Template("Cargo.workspace.toml not found".to_string()))?;

        let modules: Vec<String> = self
            .config
            .modules()
            .iter()
            .map(|m| format!("\"crates/{}\"", module_name(m)))
            .collect();

        let mut engine = TemplateEngine::new();
        engine.set("project_name", &self.config.name);
        engine.set("modules", &modules.join(", "));

        let content = engine.render(&template);
        fs::write(path.join("Cargo.toml"), content)?;
        Ok(())
    }

    fn create_github_workflows(&self, path: &Path) -> Result<()> {
        let workflow_dir = path.join(".github/workflows");
        fs::create_dir_all(&workflow_dir)?;

        for name in &["build.yml", "test.yml", "release.yml"] {
            if let Some(template) = Templates::get_template(&format!("github/{}", name)) {
                let content = self.engine.render(&template);
                fs::write(workflow_dir.join(name), content)?;
            }
        }
        Ok(())
    }

    fn create_docker(&self, path: &Path) -> Result<()> {
        if let Some(template) = Templates::get_template("docker/Dockerfile") {
            let content = self.engine.render(&template);
            fs::write(path.join("Dockerfile"), content)?;
        }

        if let Some(template) = Templates::get_template("docker/dockerignore") {
            let content = self.engine.render(&template);
            fs::write(path.join(".dockerignore"), content)?;
        }
        Ok(())
    }

    fn create_makefile(&self, path: &Path) -> Result<()> {
        if let Some(template) = Templates::get_template("base/Makefile") {
            let content = self.engine.render(&template);
            fs::write(path.join("Makefile"), content)?;
        }
        Ok(())
    }

    fn create_readme(&self, path: &Path) -> Result<()> {
        if let Some(template) = Templates::get_template("base/README.md") {
            let content = self.engine.render(&template);
            fs::write(path.join("README.md"), content)?;
        }
        Ok(())
    }

    fn create_gitignore(&self, path: &Path) -> Result<()> {
        if let Some(template) = Templates::get_template("base/gitignore") {
            let content = self.engine.render(&template);
            fs::write(path.join(".gitignore"), content)?;
        }
        Ok(())
    }

    fn create_rust_toolchain(&self, path: &Path) -> Result<()> {
        if let Some(template) = Templates::get_template("base/rust-toolchain.toml") {
            let content = self.engine.render(&template);
            fs::write(path.join("rust-toolchain.toml"), content)?;
        }
        Ok(())
    }

    fn create_proto(&self, path: &Path) -> Result<()> {
        let proto_dir = path.join("proto");
        fs::create_dir_all(&proto_dir)?;

        if let Some(template) = Templates::get_template("proto/service.proto") {
            let mut engine = TemplateEngine::new();
            engine.set("project_name", &self.config.name);
            engine.set("project_name_snake", &self.config.name.replace('-', "_"));
            engine.set("project_name_pascal", &to_pascal_case(&self.config.name));

            let content = engine.render(&template);
            fs::write(proto_dir.join("service.proto"), content)?;
        }
        Ok(())
    }

    fn create_env_example(&self, path: &Path) -> Result<()> {
        if let Some(template) = Templates::get_template("base/env.example") {
            let content = self.engine.render(&template);
            fs::write(path.join(".env.example"), content)?;
        }
        Ok(())
    }

    fn create_module(&self, path: &Path, module: &Module) -> Result<()> {
        let module_name = module_name(module);
        let module_dir = path.join("crates").join(&module_name);
        fs::create_dir_all(&module_dir)?;

        self.create_module_cargo(&module_dir, module)?;
        self.create_module_src(&module_dir, module)?;

        if *module == Module::Core {
            self.create_examples(&module_dir)?;
        }

        if *module == Module::Persistence && self.config.persistence.is_some() {
            self.create_migrations(&module_dir)?;
        }

        Ok(())
    }

    fn create_module_cargo(&self, path: &Path, module: &Module) -> Result<()> {
        let module_name_str = module_name(module);

        let template_path =
            if *module == Module::Cli && self.config.project_type == ProjectType::Service {
                "modules/cli/Cargo_service.toml".to_string()
            } else {
                format!("modules/{}/Cargo.toml", module_name_str)
            };

        if let Some(template) = Templates::get_template(&template_path) {
            let mut engine = TemplateEngine::new();
            engine.set("project_name", &self.config.name);
            engine.set("module_name", &module_name_str);

            if *module == Module::Persistence {
                let persistence_deps = match self.config.persistence {
                    Some(PersistenceType::Postgres) => {
                        r#"sqlx = { version = "0.7", features = ["runtime-tokio", "postgres"] }"#
                    }
                    Some(PersistenceType::Sqlite) => {
                        r#"sqlx = { version = "0.7", features = ["runtime-tokio", "sqlite"] }"#
                    }
                    _ => "",
                };
                engine.set("persistence_deps", persistence_deps);
            }

            if *module == Module::Api && self.config.has_grpc {
                engine.set(
                    "grpc_deps",
                    r#"tonic = "0.11"
prost = "0.12""#,
                );
                engine.set(
                    "build_deps",
                    r#"
[build-dependencies]
tonic-build = "0.11""#,
                );
            } else {
                engine.set("grpc_deps", "");
                engine.set("build_deps", "");
            }

            let content = engine.render(&template);
            fs::write(path.join("Cargo.toml"), content)?;
        }
        Ok(())
    }

    fn create_module_src(&self, path: &Path, module: &Module) -> Result<()> {
        let module_name_str = module_name(module);
        let src_dir = path.join("src");
        fs::create_dir_all(&src_dir)?;

        let main_file = if *module == Module::Cli {
            "main.rs"
        } else {
            "lib.rs"
        };

        let template_path =
            if *module == Module::Cli && self.config.project_type == ProjectType::Service {
                "modules/cli/main_service.rs".to_string()
            } else {
                format!("modules/{}/{}", module_name_str, main_file)
            };

        if let Some(template) = Templates::get_template(&template_path) {
            let content = self.engine.render(&template);
            fs::write(src_dir.join(main_file), content)?;
        }

        if *module == Module::Core {
            self.create_core_files(&src_dir)?;
        }

        if *module == Module::Api {
            self.create_api_files(&src_dir)?;
        }

        if *module == Module::Runtime {
            self.create_runtime_files(&src_dir)?;
        }

        if *module == Module::Client {
            self.create_client_files(&src_dir)?;
        }

        Ok(())
    }

    fn create_core_files(&self, src_dir: &Path) -> Result<()> {
        if let Some(template) = Templates::get_template("modules/core/error.rs") {
            let content = self.engine.render(&template);
            fs::write(src_dir.join("error.rs"), content)?;
        }
        Ok(())
    }

    fn create_api_files(&self, src_dir: &Path) -> Result<()> {
        if let Some(template) = Templates::get_template("modules/api/routes.rs") {
            let content = self.engine.render(&template);
            fs::write(src_dir.join("routes.rs"), content)?;
        }

        let handlers_dir = src_dir.join("handlers");
        fs::create_dir_all(&handlers_dir)?;

        if let Some(template) = Templates::get_template("modules/api/handlers/mod.rs") {
            let content = self.engine.render(&template);
            fs::write(handlers_dir.join("mod.rs"), content)?;
        }

        // Create build.rs for gRPC
        if self.config.has_grpc
            && let Some(template) = Templates::get_template("modules/api/build.rs")
        {
            let content = self.engine.render(&template);
            // build.rs should be in the module directory, not src
            if let Some(module_dir) = src_dir.parent() {
                fs::write(module_dir.join("build.rs"), content)?;
            }
        }

        Ok(())
    }

    fn create_runtime_files(&self, src_dir: &Path) -> Result<()> {
        if let Some(template) = Templates::get_template("modules/runtime/server.rs") {
            let content = self.engine.render(&template);
            fs::write(src_dir.join("server.rs"), content)?;
        }
        Ok(())
    }

    fn create_client_files(&self, src_dir: &Path) -> Result<()> {
        if let Some(template) = Templates::get_template("modules/client/http.rs") {
            let content = self.engine.render(&template);
            fs::write(src_dir.join("http.rs"), content)?;
        }
        Ok(())
    }

    fn create_examples(&self, module_dir: &Path) -> Result<()> {
        let examples_dir = module_dir.join("examples");
        fs::create_dir_all(&examples_dir)?;

        if let Some(template) = Templates::get_template("modules/core/examples/basic.rs") {
            let content = self.engine.render(&template);
            fs::write(examples_dir.join("basic.rs"), content)?;
        }
        Ok(())
    }

    fn create_migrations(&self, module_dir: &Path) -> Result<()> {
        let migrations_dir = module_dir.join("migrations");
        fs::create_dir_all(&migrations_dir)?;

        fs::write(migrations_dir.join(".gitkeep"), "")?;
        Ok(())
    }
}

fn module_name(module: &Module) -> String {
    match module {
        Module::Core => "core".to_string(),
        Module::Api => "api".to_string(),
        Module::Runtime => "runtime".to_string(),
        Module::Cli => "cli".to_string(),
        Module::Client => "client".to_string(),
        Module::Persistence => "persistence".to_string(),
    }
}

fn to_pascal_case(s: &str) -> String {
    s.split(['-', '_'])
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().chain(chars).collect(),
            }
        })
        .collect()
}
