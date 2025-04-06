# When type 'make' it will automatically run 'run'
.DEFAULT_GOAL := run

.PHONY: clean build run release

# Default file to run if none specified
FILE ?= ../examples/hello.mus

clean:
	cd mussel && cargo clean

build:
	cd mussel && cargo build

run:
	cd mussel && cargo run -- $(FILE)

release:
	cd mussel && cargo build --release