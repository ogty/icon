define release_title
Release v${version}
endef

export release_title

app_name     := $(shell   \
	cat Cargo.toml        \
	| grep '^name'        \
	| cut -d '=' -f2      \
	| sed -r 's/( |")//g' \
)
version      ?= $(shell   \
	cat Cargo.toml        \
	| grep '^version'     \
	| cut -d '=' -f2      \
	| sed -r 's/( |")//g' \
)
next_version := $(shell           \
	echo ${version}               \
	| awk '{                      \
		split($$0, version, "."); \
		printf(                   \
			"%s.%s.%s",           \
			version[1],           \
			version[2],           \
			version[3] + 1        \
		);                        \
	}'                            \
)
target       := x86_64-apple-darwin
formula_url  := https://raw.githubusercontent.com/ogty/homebrew-icon/main/Formula/icon.rb
tar_file     := ${app_name}-${version}-${target}.tar.gz
repository   := https:\/\/github.com\/ogty\/icon
#                     ^^^^          ^^    ^^                     WARNING: escape slash
download_url := ${repository}\/releases\/download\/v${version}\/${tar_file}
#                            ^^        ^^        ^^           ^^ WARNING: escape slash

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
	&& mv target/debug/${app_name} ./${app_name}

# 1. Update version in Cargo.toml
# 2. Build release binary
# 3. Create tar.gz
# 4. Create release on GitHub
# 5. Create formula for Homebrew
# 6. Clean up
update:
	@make update-version  \
	&& make release-build \
	&& make tar           \
	&& make release       \
	&& make formula       \
	&& make tar-clean

formula:
	@curl -s ${formula_url} \
	| sed -r 's/^  version ".*"$$/  version "${version}"/g' \
	| sed -r 's/^  url ".*"$$/  url "${download_url}"/g'    \
	| sed -r 's/^  sha256 ".*"$$/  sha256 "$(shell          \
		shasum -a 256 target/release/${tar_file}            \
		| cut -d ' ' -f1)"/g'                               \
	| pbcopy

show:
	@cat Makefile         \
	| grep -E '^[-a-z]+:' \
	| sed -r 's/(.+):/- \1/g'

release:
	@gh release create v${version} target/release/${tar_file} --title "$$release_title" --latest

release-build:
	@cargo build --release

tar:
	@tar -czf target/release/${tar_file} target/release/${app_name}

tar-clean:
	@rm target/release/*.tar.gz

update-version:
	@cat Cargo.toml                                                      \
	| sed -r 's/^version = "${version}"$$/version = "${next_version}"/g' \
	> Cargo.toml.tmp                                                     \
	&& mv Cargo.toml.tmp Cargo.toml
