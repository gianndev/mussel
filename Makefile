# When type 'make' it will automatically run 'run'
.DEFAULT_GOAL := run

.PHONY: clean build run release

clean:
	cd mussel && cargo clean

build:
	cd mussel && cargo build

run:
	cd mussel && cargo run

release:
	cd mussel && cargo build --release