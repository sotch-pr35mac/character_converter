.DEFAULT_GOAL := build

build:
	cargo build

test:
	cargo test

format:
	cargo fmt

benchmark:
	cargo +nightly bench --feature=bench
