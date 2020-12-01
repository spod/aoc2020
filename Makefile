.PHONY:	default
default:	fmt build run

# TODO source from os env
day = 02

build:
	#@cargo build --quiet
	cargo build

run:
	#@cargo run  --quiet --bin aoc $(day)
	cargo run --bin aoc $(day)

test:
	cargo test --package day$(day)

fmt:
	@cargo fmt
