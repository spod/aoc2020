.PHONY:	default
default:	fmt build run

# TODO source from os env
day = 08

build:
	#@cargo build --quiet
	cargo build

run:
	#@cargo run  --quiet --bin aoc $(day)
	#cargo run --bin aoc $(day)
	cd target && ./debug/aoc $(day)

test:
	cargo test --package day$(day)

fmt:
	@cargo fmt
