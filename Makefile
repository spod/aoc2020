.PHONY:	default
default:	fmt build run

day = 1

build:
	#@cargo build --quiet
	cargo build

run:
	@cargo run  --quiet --bin aoc $(day)

fmt:
	@cargo fmt
