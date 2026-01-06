#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProjectType {
    Service,
    Cli,
    Lib,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PersistenceType {
    Postgres,
    Sqlite,
    File,
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
    pub has_grpc: bool,
    pub has_http: bool,
    pub has_client: bool,
}

impl ProjectConfig {
    pub fn modules(&self) -> Vec<Module> {
        match self.project_type {
            ProjectType::Service => {
                let mut modules = vec![
                    Module::Core,
                    Module::Api,
                    Module::Runtime,
                    Module::Cli,
                ];
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
        }
    }
}
