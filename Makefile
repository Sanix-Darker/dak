.PHONY: format build run

format:
	cargo fmt

build:
	cargo build

run: build
	cargo run run --image xxxx

pull: build
	cargo run pull --image yyyy

zigzag: build
	cargo run zigzag --image yyyy
