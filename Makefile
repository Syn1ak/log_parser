all: fmt lint build test

fmt:
	cargo fmt

lint:
	cargo clippy

build:
	cargo build

test:
	cargo test

# 'inputs/logs.txt'
run:
	cargo run -- $(FILE)

help:
	@echo "Available commands:"
	@echo "make all     - Formats, lints, builds, and tests the project"
	@echo "make fmt     - Runs cargo fmt for formatting"
	@echo "make lint    - Runs cargo clippy for linting"
	@echo "make build   - Builds the project"
	@echo "make test    - Runs tests"
	@echo "make run FILE=path/to/your_file.txt - Runs the parser with a specified file"
