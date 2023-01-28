version ?= 0.0.1

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
	@cargo build --release \
	&& make tar \
	&& make hash

tar:
	@cd target/release \
	&& tar -czf icon-${version}-x86_64-apple-darwin.tar.gz icon

hash:
	@cd target/release                                          \
	&& shasum -a 256 icon-${version}-x86_64-apple-darwin.tar.gz \
	| awk '{printf $$1}'                                        \
	| pbcopy

tar-clean:
	@rm target/release/*.tar.gz
