use crate::config::{FrontendType, Module, PersistenceType, ProjectConfig, ProjectType};
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

        if self.config.frontend.is_some() {
            self.create_frontend(&project_path)?;
            self.create_docker_compose(&project_path)?;
        }

        for module in self.config.modules() {
            self.create_module(&project_path, &module)?;
        }

        // Sample project specific files
        if self.config.project_type == ProjectType::Sample {
            self.create_sample_files(&project_path)?;
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

        // Build authors string: "Name <email>" or empty array
        let authors = if !self.config.author_name.is_empty() || !self.config.author_email.is_empty()
        {
            let author = if !self.config.author_name.is_empty()
                && !self.config.author_email.is_empty()
            {
                format!(
                    "{} <{}>",
                    self.config.author_name, self.config.author_email
                )
            } else if !self.config.author_name.is_empty() {
                self.config.author_name.clone()
            } else {
                format!("<{}>", self.config.author_email)
            };
            format!("\"{}\"", author)
        } else {
            String::new()
        };

        // Build repository string: empty if no author info
        let repository = if !self.config.author_name.is_empty() {
            format!("https://github.com/{}", self.config.name)
        } else {
            String::new()
        };

        let mut engine = TemplateEngine::new();
        engine.set("project_name", &self.config.name);
        engine.set("modules", &modules.join(", "));
        engine.set("authors", &authors);
        engine.set("repository", &repository);

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
        // Use sample-specific Makefile for Sample projects
        let template_path = if self.config.project_type == ProjectType::Sample {
            "samples/Makefile"
        } else {
            "base/Makefile"
        };

        if let Some(template) = Templates::get_template(template_path) {
            let content = self.engine.render(&template);
            fs::write(path.join("Makefile"), content)?;
        }
        Ok(())
    }

    fn create_readme(&self, path: &Path) -> Result<()> {
        // Use sample-specific README for Sample projects
        let template_path = if self.config.project_type == ProjectType::Sample {
            "samples/README.md"
        } else {
            "base/README.md"
        };

        if let Some(template) = Templates::get_template(template_path) {
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

        let template_path = if *module == Module::Cli {
            match self.config.project_type {
                ProjectType::Sample => "samples/cli/Cargo.toml".to_string(),
                ProjectType::Service => "modules/cli/Cargo_service.toml".to_string(),
                _ => format!("modules/{}/Cargo.toml", module_name_str),
            }
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

        let template_path = if *module == Module::Cli {
            match self.config.project_type {
                ProjectType::Sample => "samples/cli/main.rs".to_string(),
                ProjectType::Service => "modules/cli/main_service.rs".to_string(),
                _ => format!("modules/{}/{}", module_name_str, main_file),
            }
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
        let (routes_path, handlers_path) = if self.config.project_type == ProjectType::Sample {
            ("samples/api/routes.rs", "samples/api/handlers/mod.rs")
        } else {
            ("modules/api/routes.rs", "modules/api/handlers/mod.rs")
        };

        if let Some(template) = Templates::get_template(routes_path) {
            let content = self.engine.render(&template);
            fs::write(src_dir.join("routes.rs"), content)?;
        }

        let handlers_dir = src_dir.join("handlers");
        fs::create_dir_all(&handlers_dir)?;

        if let Some(template) = Templates::get_template(handlers_path) {
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

    fn create_frontend(&self, path: &Path) -> Result<()> {
        let frontend_type = match self.config.frontend {
            Some(FrontendType::Spa) => "spa",
            Some(FrontendType::Ssr) => "ssr",
            None => return Ok(()),
        };

        let frontend_dir = path.join("frontend");
        fs::create_dir_all(&frontend_dir)?;

        // Copy frontend template files
        let template_prefix = format!("frontend/{}/", frontend_type);

        // package.json
        if let Some(template) = Templates::get_template(&format!("{}package.json", template_prefix))
        {
            let content = self.engine.render(&template);
            fs::write(frontend_dir.join("package.json"), content)?;
        }

        // tsconfig.json
        if let Some(template) =
            Templates::get_template(&format!("{}tsconfig.json", template_prefix))
        {
            fs::write(frontend_dir.join("tsconfig.json"), template)?;
        }

        // Dockerfile
        if let Some(template) = Templates::get_template(&format!("{}Dockerfile", template_prefix)) {
            fs::write(frontend_dir.join("Dockerfile"), template)?;
        }

        // .dockerignore
        if let Some(template) = Templates::get_template(&format!("{}dockerignore", template_prefix))
        {
            fs::write(frontend_dir.join(".dockerignore"), template)?;
        }

        match self.config.frontend {
            Some(FrontendType::Spa) => self.create_spa_files(&frontend_dir)?,
            Some(FrontendType::Ssr) => self.create_ssr_files(&frontend_dir)?,
            None => {}
        }

        Ok(())
    }

    fn create_spa_files(&self, frontend_dir: &Path) -> Result<()> {
        // vite.config.ts
        if let Some(template) = Templates::get_template("frontend/spa/vite.config.ts") {
            fs::write(frontend_dir.join("vite.config.ts"), template)?;
        }

        // index.html
        if let Some(template) = Templates::get_template("frontend/spa/index.html") {
            let content = self.engine.render(&template);
            fs::write(frontend_dir.join("index.html"), content)?;
        }

        // nginx.conf
        if let Some(template) = Templates::get_template("frontend/spa/nginx.conf") {
            fs::write(frontend_dir.join("nginx.conf"), template)?;
        }

        // src directory
        let src_dir = frontend_dir.join("src");
        fs::create_dir_all(&src_dir)?;

        if let Some(template) = Templates::get_template("frontend/spa/src/main.tsx") {
            fs::write(src_dir.join("main.tsx"), template)?;
        }

        if let Some(template) = Templates::get_template("frontend/spa/src/App.tsx") {
            let content = self.engine.render(&template);
            fs::write(src_dir.join("App.tsx"), content)?;
        }

        if let Some(template) = Templates::get_template("frontend/spa/src/vite-env.d.ts") {
            fs::write(src_dir.join("vite-env.d.ts"), template)?;
        }

        Ok(())
    }

    fn create_ssr_files(&self, frontend_dir: &Path) -> Result<()> {
        // next.config.ts
        if let Some(template) = Templates::get_template("frontend/ssr/next.config.ts") {
            fs::write(frontend_dir.join("next.config.ts"), template)?;
        }

        // app directory
        let app_dir = frontend_dir.join("app");
        fs::create_dir_all(&app_dir)?;

        if let Some(template) = Templates::get_template("frontend/ssr/app/layout.tsx") {
            let content = self.engine.render(&template);
            fs::write(app_dir.join("layout.tsx"), content)?;
        }

        if let Some(template) = Templates::get_template("frontend/ssr/app/page.tsx") {
            let content = self.engine.render(&template);
            fs::write(app_dir.join("page.tsx"), content)?;
        }

        if let Some(template) = Templates::get_template("frontend/ssr/app/globals.css") {
            fs::write(app_dir.join("globals.css"), template)?;
        }

        Ok(())
    }

    fn create_docker_compose(&self, path: &Path) -> Result<()> {
        if let Some(template) = Templates::get_template("base/docker-compose.yml") {
            let frontend_service = match self.config.frontend {
                Some(FrontendType::Spa) => {
                    Templates::get_template("frontend/spa/docker-compose.service.yml")
                        .unwrap_or_default()
                }
                Some(FrontendType::Ssr) => {
                    Templates::get_template("frontend/ssr/docker-compose.service.yml")
                        .unwrap_or_default()
                }
                None => String::new(),
            };

            let mut engine = TemplateEngine::new();
            engine.set("frontend_service", &frontend_service);

            let content = engine.render(&template);
            fs::write(path.join("docker-compose.yml"), content)?;
        }
        Ok(())
    }

    /// Create sample project specific files (board application)
    fn create_sample_files(&self, path: &Path) -> Result<()> {
        // Create board module in core
        self.create_board_module(path)?;

        // Create E2E test directory
        self.create_e2e_tests(path)?;

        // Create docs directory
        self.create_sample_docs(path)?;

        // Create sample-specific docker-compose with MinIO
        self.create_sample_docker_compose(path)?;

        Ok(())
    }

    fn create_board_module(&self, path: &Path) -> Result<()> {
        let board_dir = path.join("crates/core/src/board");
        fs::create_dir_all(&board_dir)?;

        // board/mod.rs
        if let Some(template) = Templates::get_template("samples/board/mod.rs") {
            let content = self.engine.render(&template);
            fs::write(board_dir.join("mod.rs"), content)?;
        }

        // board/models.rs
        if let Some(template) = Templates::get_template("samples/board/models.rs") {
            let content = self.engine.render(&template);
            fs::write(board_dir.join("models.rs"), content)?;
        }

        // board/permission.rs
        if let Some(template) = Templates::get_template("samples/board/permission.rs") {
            let content = self.engine.render(&template);
            fs::write(board_dir.join("permission.rs"), content)?;
        }

        Ok(())
    }

    fn create_e2e_tests(&self, path: &Path) -> Result<()> {
        let e2e_dir = path.join("e2e");
        fs::create_dir_all(&e2e_dir)?;

        // playwright.config.ts
        if let Some(template) = Templates::get_template("samples/e2e/playwright.config.ts") {
            let content = self.engine.render(&template);
            fs::write(e2e_dir.join("playwright.config.ts"), content)?;
        }

        // package.json
        if let Some(template) = Templates::get_template("samples/e2e/package.json") {
            let content = self.engine.render(&template);
            fs::write(e2e_dir.join("package.json"), content)?;
        }

        // helpers directory
        let helpers_dir = e2e_dir.join("helpers");
        fs::create_dir_all(&helpers_dir)?;

        if let Some(template) = Templates::get_template("samples/e2e/helpers/auth.ts") {
            fs::write(helpers_dir.join("auth.ts"), template)?;
        }

        // tests directory
        let tests_dir = e2e_dir.join("tests");
        fs::create_dir_all(&tests_dir)?;

        if let Some(template) = Templates::get_template("samples/e2e/tests/posts.spec.ts") {
            fs::write(tests_dir.join("posts.spec.ts"), template)?;
        }

        // fixtures directory
        let fixtures_dir = e2e_dir.join("fixtures");
        fs::create_dir_all(&fixtures_dir)?;
        fs::write(fixtures_dir.join(".gitkeep"), "")?;

        Ok(())
    }

    fn create_sample_docs(&self, path: &Path) -> Result<()> {
        let docs_dir = path.join("docs");
        fs::create_dir_all(&docs_dir)?;

        // docs/api.md
        if let Some(template) = Templates::get_template("samples/docs/api.md") {
            let content = self.engine.render(&template);
            fs::write(docs_dir.join("api.md"), content)?;
        }

        // docs/architecture.md
        if let Some(template) = Templates::get_template("samples/docs/architecture.md") {
            let content = self.engine.render(&template);
            fs::write(docs_dir.join("architecture.md"), content)?;
        }

        // docs/e2e-testing.md
        if let Some(template) = Templates::get_template("samples/docs/e2e-testing.md") {
            let content = self.engine.render(&template);
            fs::write(docs_dir.join("e2e-testing.md"), content)?;
        }

        Ok(())
    }

    fn create_sample_docker_compose(&self, path: &Path) -> Result<()> {
        // Override docker-compose with sample version (includes MinIO)
        if let Some(template) = Templates::get_template("samples/docker-compose.yml") {
            let content = self.engine.render(&template);
            fs::write(path.join("docker-compose.yml"), content)?;
        }
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
