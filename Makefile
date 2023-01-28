run:
	cargo run

format:
	cargo fmt

lint:
	cargo clippy

test:
	cargo test

build:
	@cargo build \
	&& mv target/debug/icon ./icon

release:
	cargo build --release
