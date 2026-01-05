.PHONY: build test run fmt clippy clean

build:
	cargo build --all

test:
	cargo test --all

run:
	cargo run -p {{project_name}}-cli

fmt:
	cargo fmt --all

clippy:
	cargo clippy --all -- -D warnings

clean:
	cargo clean
