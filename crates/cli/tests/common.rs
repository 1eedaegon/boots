use std::path::{Path, PathBuf};
use std::process::{Child, Command, Output, Stdio};
use std::time::Duration;
use tempfile::TempDir;

pub struct TempProject {
    dir: TempDir,
}

impl TempProject {
    pub fn new() -> Self {
        Self {
            dir: TempDir::new().expect("Failed to create temp directory"),
        }
    }

    pub fn path(&self) -> &Path {
        self.dir.path()
    }
}

impl Default for TempProject {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(dead_code)]
pub struct CommandResult {
    pub success: bool,
    pub stdout: String,
    pub stderr: String,
}

impl From<Output> for CommandResult {
    fn from(output: Output) -> Self {
        Self {
            success: output.status.success(),
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        }
    }
}

pub fn run_boots_command(args: &[&str], cwd: &Path) -> CommandResult {
    let boots_bin = env!("CARGO_BIN_EXE_cargo-boots");

    let output = Command::new(boots_bin)
        .args(args)
        .current_dir(cwd)
        .output()
        .expect("Failed to run boots");

    CommandResult::from(output)
}

pub fn cargo_build(project_path: &Path) -> CommandResult {
    let output = Command::new("cargo")
        .args(["build", "--all"])
        .current_dir(project_path)
        .output()
        .expect("Failed to run cargo build");

    CommandResult::from(output)
}

pub fn cargo_test(project_path: &Path) -> CommandResult {
    let output = Command::new("cargo")
        .args(["test", "--all"])
        .current_dir(project_path)
        .output()
        .expect("Failed to run cargo test");

    CommandResult::from(output)
}

pub fn project_path(temp: &TempProject, name: &str) -> PathBuf {
    temp.path().join(name)
}

pub fn start_server(project_path: &Path, project_name: &str, port: u16) -> Child {
    Command::new("cargo")
        .args([
            "run",
            "-p",
            &format!("{}-cli", project_name),
            "--",
            "--port",
            &port.to_string(),
        ])
        .current_dir(project_path)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start server")
}

pub async fn wait_for_server(url: &str, max_wait: Duration) -> bool {
    let client = reqwest::Client::new();
    let start = std::time::Instant::now();

    while start.elapsed() < max_wait {
        if let Ok(resp) = client.get(url).send().await {
            if resp.status().is_success() {
                return true;
            }
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
    false
}
