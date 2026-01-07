.PHONY: build test test-unit test-integration run fmt clippy clean install

build:
	cargo build --all

test: test-unit test-integration

test-unit:
	cargo test --lib --all

test-integration:
	cargo test --test integration -- --test-threads=1

run:
	cargo run -p boots -- --help

fmt:
	cargo fmt --all

clippy:
	cargo clippy --all -- -D warnings

clean:
	cargo clean

install:
	cargo install --path crates/cli
