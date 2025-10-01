use std::{
    fs::{self, create_dir_all, read_to_string, write},
    path::Path,
};

use toml_edit::{ArrayOfTables, DocumentMut, Item, Table, value};

use crate::{BootsError, Result};

pub fn add(target: &str) -> Result<()> {
    match target {
        "gh:test" => add_github_test()?,
        "gh:build" => add_github_build()?,
        "gh:semver" => add_github_semver()?,
        "test:perf" => add_test_perf()?,
        _ => return Err(BootsError::Other(format!("Unknown target: {}", target))),
    }

    Ok(())
}

fn add_github_test() -> Result<()> {
    let workflow_path = ".github/workflows/test.yml";
    if Path::new(workflow_path).exists() {
        return Err(BootsError::AlreadyExists(workflow_path.to_string()));
    }
    let content = include_str!("../../../template/.github/workflows/test.yml");
    create_dir_all(".github/workflows")?;
    write(workflow_path, content)?;
    Ok(())
}

fn add_github_build() -> Result<()> {
    let workflow_path = ".github/workflows/build.yml";
    if Path::new(workflow_path).exists() {
        return Err(BootsError::AlreadyExists(workflow_path.to_string()));
    }
    let content = include_str!("../../../template/.github/workflows/build.yml");
    create_dir_all(".github/workflows")?;
    write(workflow_path, content)?;
    Ok(())
}

fn add_github_semver() -> Result<()> {
    let workflow_path = ".github/workflows/semver.yml";
    if Path::new(workflow_path).exists() {
        return Err(BootsError::AlreadyExists(workflow_path.to_string()));
    }
    let workflow_content = include_str!("../../../template/.github/workflows/semver.yml");
    create_dir_all(".github/workflows")?;
    write(workflow_path, workflow_content)?;
    Ok(())
}

fn add_test_perf() -> Result<()> {
    let bench_dir = "crates/core/benches";
    let bench_file = "crates/core/benches/benchmark.rs";

    if Path::new(bench_file).exists() {
        return Err(BootsError::AlreadyExists(bench_file.to_string()));
    }

    // Cargo.toml 수정
    add_criterion_to_cargo()?;

    // 벤치마크 파일 생성
    let bench_content = include_str!("../../../template/benches/benchmark.rs");

    create_dir_all(bench_dir)?;
    write(bench_file, bench_content)?;
    Ok(())
}

fn add_criterion_to_cargo() -> Result<()> {
    let target_cargo_toml_path = "crates/core/Cargo.toml";
    let target_content = read_to_string(target_cargo_toml_path)?;
    let mut target_cargo_toml = target_content
        .parse::<DocumentMut>()
        .map_err(|e| BootsError::Other(format!("Failed to parse Cargo.toml: {}", e)))?;

    if !target_cargo_toml.contains_key("dev-dependencies") {
        target_cargo_toml["dev-dependencies"] = toml_edit::table();
    }

    let dev_deps = target_cargo_toml["dev-dependencies"]
        .as_table_mut()
        .ok_or_else(|| BootsError::Other("dev-dependencies is not a table".to_string()))?;

    if !dev_deps.contains_key("criterion") {
        dev_deps["criterion"] = value("1.0");
    }

    // [[bench]] 섹션 추가 (이미 있는지 확인)
    if !target_cargo_toml.contains_key("bench") {
        let mut bench_array = ArrayOfTables::new();
        let mut bench_table = Table::new();
        bench_table["name"] = value("benchmark");
        bench_table["harness"] = value(false);
        bench_array.push(bench_table);

        target_cargo_toml["bench"] = Item::ArrayOfTables(bench_array);
    }

    fs::write(target_cargo_toml_path, target_cargo_toml.to_string())?;

    Ok(())
}
