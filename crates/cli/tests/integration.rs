mod common;

use common::*;
use std::process::Command;
use std::time::Duration;

// Service Tests

#[test]
fn test_service_generation() {
    let temp = TempProject::new();
    let result = run_boots_command(&["boots", "service", "test-service"], temp.path());
    assert!(result.success, "Generation failed: {}", result.stderr);

    let project = project_path(&temp, "test-service");

    assert!(project.join("crates/api").exists(), "api module missing");
    assert!(
        project.join("crates/runtime").exists(),
        "runtime module missing"
    );
    assert!(project.join("crates/cli").exists(), "cli module missing");
    assert!(project.join("crates/core").exists(), "core module missing");
    assert!(project.join("Dockerfile").exists(), "Dockerfile missing");
    assert!(project.join("Makefile").exists(), "Makefile missing");
}

#[test]
fn test_service_builds() {
    let temp = TempProject::new();
    run_boots_command(&["boots", "service", "test-svc-build"], temp.path());

    let project = project_path(&temp, "test-svc-build");

    let build = cargo_build(&project);
    assert!(build.success, "Build failed: {}", build.stderr);
}

#[test]
fn test_service_with_postgres() {
    let temp = TempProject::new();
    let result = run_boots_command(
        &["boots", "service", "test-pg", "--options", "postgres"],
        temp.path(),
    );
    assert!(result.success, "Generation failed: {}", result.stderr);

    let project = project_path(&temp, "test-pg");

    assert!(
        project.join("crates/persistence").exists(),
        "persistence module missing"
    );

    let cargo_content =
        std::fs::read_to_string(project.join("crates/persistence/Cargo.toml")).unwrap();
    assert!(
        cargo_content.contains("sqlx"),
        "sqlx dependency missing in persistence"
    );
}

// CLI Tests

#[test]
fn test_cli_generation() {
    let temp = TempProject::new();
    let result = run_boots_command(&["boots", "cli", "test-cli"], temp.path());
    assert!(result.success, "Generation failed: {}", result.stderr);

    let project = project_path(&temp, "test-cli");

    assert!(project.join("crates/core").exists(), "core module missing");
    assert!(project.join("crates/cli").exists(), "cli module missing");
    assert!(
        !project.join("crates/api").exists(),
        "api module should not exist"
    );
    assert!(
        !project.join("crates/runtime").exists(),
        "runtime module should not exist"
    );
}

#[test]
fn test_cli_builds() {
    let temp = TempProject::new();
    run_boots_command(&["boots", "cli", "test-cli-build"], temp.path());

    let project = project_path(&temp, "test-cli-build");

    let build = cargo_build(&project);
    assert!(build.success, "Build failed: {}", build.stderr);
}

#[test]
fn test_cli_help() {
    let temp = TempProject::new();
    run_boots_command(&["boots", "cli", "test-cli-help"], temp.path());

    let project = project_path(&temp, "test-cli-help");

    cargo_build(&project);

    let output = Command::new("cargo")
        .args(["run", "-p", "test-cli-help-cli", "--", "--help"])
        .current_dir(&project)
        .output()
        .unwrap();

    assert!(output.status.success(), "CLI --help failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Usage"), "Help output missing Usage");
}

#[test]
fn test_cli_with_client() {
    let temp = TempProject::new();
    let result = run_boots_command(
        &["boots", "cli", "test-cli-client", "--options", "client"],
        temp.path(),
    );
    assert!(result.success, "Generation failed: {}", result.stderr);

    let project = project_path(&temp, "test-cli-client");

    assert!(
        project.join("crates/client").exists(),
        "client module missing"
    );

    let build = cargo_build(&project);
    assert!(build.success, "Build failed: {}", build.stderr);
}

// Lib Tests

#[test]
fn test_lib_generation() {
    let temp = TempProject::new();
    let result = run_boots_command(&["boots", "lib", "test-lib"], temp.path());
    assert!(result.success, "Generation failed: {}", result.stderr);

    let project = project_path(&temp, "test-lib");

    assert!(project.join("crates/core").exists(), "core module missing");
    assert!(
        project.join("crates/core/examples").exists(),
        "examples directory missing"
    );
    assert!(
        !project.join("crates/cli").exists(),
        "cli module should not exist"
    );
}

#[test]
fn test_lib_builds() {
    let temp = TempProject::new();
    run_boots_command(&["boots", "lib", "test-lib-build"], temp.path());

    let project = project_path(&temp, "test-lib-build");

    let build = cargo_build(&project);
    assert!(build.success, "Build failed: {}", build.stderr);
}

#[test]
fn test_lib_tests_pass() {
    let temp = TempProject::new();
    run_boots_command(&["boots", "lib", "test-lib-test"], temp.path());

    let project = project_path(&temp, "test-lib-test");

    cargo_build(&project);

    let test = cargo_test(&project);
    assert!(test.success, "Tests failed: {}", test.stderr);
}

// Runtime Server Test

#[tokio::test]
async fn test_service_runtime() {
    let temp = TempProject::new();
    let result = run_boots_command(&["boots", "service", "test-runtime"], temp.path());
    assert!(result.success, "Generation failed: {}", result.stderr);

    let project = project_path(&temp, "test-runtime");

    // Build the project
    let build = cargo_build(&project);
    assert!(build.success, "Build failed: {}", build.stderr);

    // Start the server
    let port = 18080u16;
    let mut server = start_server(&project, "test-runtime", port);

    // Wait for server to be ready
    let health_url = format!("http://127.0.0.1:{}/health", port);
    let ready = wait_for_server(&health_url, Duration::from_secs(30)).await;
    assert!(ready, "Server did not start within 30 seconds");

    // Test health endpoint
    let client = reqwest::Client::new();
    let health_resp = client.get(&health_url).send().await.unwrap();
    assert_eq!(health_resp.status(), 200);

    // Test metrics endpoint
    let metrics_url = format!("http://127.0.0.1:{}/metrics", port);
    let metrics_resp = client.get(&metrics_url).send().await.unwrap();
    assert_eq!(metrics_resp.status(), 200);

    // Cleanup
    server.kill().ok();
}

// gRPC Test

#[test]
fn test_service_with_grpc() {
    let temp = TempProject::new();
    let result = run_boots_command(
        &["boots", "service", "test-grpc", "--options", "grpc"],
        temp.path(),
    );
    assert!(result.success, "Generation failed: {}", result.stderr);

    let project = project_path(&temp, "test-grpc");

    // Verify proto directory exists
    assert!(project.join("proto").exists(), "proto directory missing");
    assert!(
        project.join("proto/service.proto").exists(),
        "service.proto missing"
    );

    // Verify build.rs exists in api module
    assert!(
        project.join("crates/api/build.rs").exists(),
        "build.rs missing in api module"
    );

    // Verify tonic dependency in api Cargo.toml
    let cargo_content = std::fs::read_to_string(project.join("crates/api/Cargo.toml")).unwrap();
    assert!(
        cargo_content.contains("tonic"),
        "tonic dependency missing in api"
    );
    assert!(
        cargo_content.contains("tonic-build"),
        "tonic-build dependency missing in api"
    );
}

#[test]
fn test_service_with_postgres_env() {
    let temp = TempProject::new();
    let result = run_boots_command(
        &["boots", "service", "test-pg-env", "--options", "postgres"],
        temp.path(),
    );
    assert!(result.success, "Generation failed: {}", result.stderr);

    let project = project_path(&temp, "test-pg-env");

    // Verify .env.example exists
    assert!(
        project.join(".env.example").exists(),
        ".env.example missing"
    );

    // Verify .env.example content
    let env_content = std::fs::read_to_string(project.join(".env.example")).unwrap();
    assert!(
        env_content.contains("DATABASE_URL"),
        "DATABASE_URL missing in .env.example"
    );
}

// Sample Project Tests

#[test]
fn test_sample_generation() {
    let temp = TempProject::new();
    let result = run_boots_command(
        &["boots", "sample", "test-board", "--options", "sample"],
        temp.path(),
    );
    assert!(result.success, "Generation failed: {}", result.stderr);

    let project = project_path(&temp, "test-board");

    // Verify basic structure
    assert!(project.join("crates/core").exists(), "core module missing");
    assert!(project.join("crates/api").exists(), "api module missing");
    assert!(
        project.join("crates/runtime").exists(),
        "runtime module missing"
    );
    assert!(project.join("crates/cli").exists(), "cli module missing");
    assert!(
        project.join("crates/persistence").exists(),
        "persistence module missing"
    );

    // Verify sample-specific files
    assert!(
        project.join("crates/core/src/board").exists(),
        "board module missing"
    );
    assert!(
        project.join("crates/core/src/board/mod.rs").exists(),
        "board/mod.rs missing"
    );
    assert!(
        project.join("crates/core/src/board/models.rs").exists(),
        "board/models.rs missing"
    );
    assert!(
        project.join("crates/core/src/board/permission.rs").exists(),
        "board/permission.rs missing"
    );

    // Verify E2E test directory
    assert!(project.join("e2e").exists(), "e2e directory missing");
    assert!(
        project.join("e2e/playwright.config.ts").exists(),
        "playwright.config.ts missing"
    );
    assert!(
        project.join("e2e/package.json").exists(),
        "e2e/package.json missing"
    );

    // Verify docs directory
    assert!(project.join("docs").exists(), "docs directory missing");
    assert!(project.join("docs/api.md").exists(), "docs/api.md missing");
    assert!(
        project.join("docs/architecture.md").exists(),
        "docs/architecture.md missing"
    );

    // Verify frontend (SPA by default)
    assert!(
        project.join("frontend").exists(),
        "frontend directory missing"
    );

    // Verify docker-compose with MinIO
    let compose_content = std::fs::read_to_string(project.join("docker-compose.yml")).unwrap();
    assert!(
        compose_content.contains("minio"),
        "minio service missing in docker-compose"
    );
}

#[test]
fn test_sample_default_options() {
    let temp = TempProject::new();
    // Without explicit options, sample should still work
    let result = run_boots_command(&["boots", "sample", "test-board-default"], temp.path());
    assert!(result.success, "Generation failed: {}", result.stderr);

    let project = project_path(&temp, "test-board-default");
    assert!(project.join("crates/core").exists(), "core module missing");
}

#[test]
fn test_sample_ignores_other_options() {
    let temp = TempProject::new();
    // sample option should ignore other options like sqlite
    let result = run_boots_command(
        &[
            "boots",
            "sample",
            "test-board-ignore",
            "--options",
            "sample,sqlite",
        ],
        temp.path(),
    );
    assert!(result.success, "Generation failed: {}", result.stderr);

    let project = project_path(&temp, "test-board-ignore");

    // Should use postgres (sample default), not sqlite
    let compose_content = std::fs::read_to_string(project.join("docker-compose.yml")).unwrap();
    assert!(
        compose_content.contains("postgres"),
        "postgres service missing"
    );
}
