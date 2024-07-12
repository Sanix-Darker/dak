.PHONY: format build run

format:
	cargo fmt

build:
	cargo build

run: build
	cargo run
