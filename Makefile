.PHONY: format build run zigzag

format:
	cargo fmt

build:
	cargo build

run: build
	cargo run run --image xxxx

pull: build
	cargo run pull --image yyyy

# will failed, showed as a wrong param for clap
zigzag: build
	cargo run zigzag --image yyyy
