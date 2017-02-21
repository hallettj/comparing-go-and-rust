.PHONY: all go rust

all: go rust

go:
	make -C go

rust:
	make -C rust
