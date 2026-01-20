#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProjectType {
    Service,
    Cli,
    Lib,
    Sample,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PersistenceType {
    Postgres,
    Sqlite,
    File,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrontendType {
    /// SPA: React + Vite, served by Nginx
    Spa,
    /// SSR: Next.js 15 with App Router
    Ssr,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Module {
    Core,
    Api,
    Runtime,
    Cli,
    Client,
    Persistence,
}

#[derive(Debug, Clone)]
pub struct ProjectConfig {
    pub name: String,
    pub project_type: ProjectType,
    pub persistence: Option<PersistenceType>,
    pub frontend: Option<FrontendType>,
    pub has_grpc: bool,
    pub has_http: bool,
    pub has_client: bool,
    pub author_name: String,
    pub author_email: String,
}

impl ProjectConfig {
    pub fn modules(&self) -> Vec<Module> {
        match self.project_type {
            ProjectType::Service => {
                let mut modules = vec![Module::Core, Module::Api, Module::Runtime, Module::Cli];
                if self.persistence.is_some() {
                    modules.push(Module::Persistence);
                }
                modules
            }
            ProjectType::Cli => {
                let mut modules = vec![Module::Core, Module::Cli];
                if self.has_client {
                    modules.push(Module::Client);
                }
                if self.persistence.is_some() {
                    modules.push(Module::Persistence);
                }
                modules
            }
            ProjectType::Lib => vec![Module::Core],
            // Sample includes all modules for full-stack board application
            ProjectType::Sample => {
                vec![
                    Module::Core,
                    Module::Api,
                    Module::Runtime,
                    Module::Cli,
                    Module::Persistence,
                ]
            }
        }
    }
}
