.PHONY: all build shell test

all: test

# Build a docker image that includes Go
build:
	docker build -t sitrus/go-comparison .

# Run `go test` in a docker container containing project files
test: build
	docker run -ti sitrus/go-comparison

# Run an interactive shell in a docker container so that you can execute `go`
# commands directly.
shell: build
	docker run -ti sitrus/go-comparison sh
