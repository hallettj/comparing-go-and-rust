.PHONY: all build shell test

all: test

# Build a docker image that includes Rust; add project files; prefetch dependencies
build:
	docker build -t sitrus/rust-comparison .

# Run `cargo test` in a docker container containing project files
test: build
	docker run -ti sitrus/rust-comparison

# Run an interactive shell in a docker container so that you can execute `cargo`
# commands directly.
shell: build
	docker run -ti sitrus/rust-comparison bash
