check:
	rustup run nightly cargo check
	rustup run nightly cargo check --tests
	rustup run nightly cargo check --examples

test:
	rustup run nightly cargo test

run:
	rustup run nightly cargo run

build:
	rustup run nightly cargo build --release
