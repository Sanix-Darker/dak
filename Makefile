.PHONY: build run format

format:
	cargo fmt

build:
   	 cargo build

run: build
	 cargo run
