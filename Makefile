check:
	rustup run nightly cargo check
	rustup run nightly cargo check --tests
	rustup run nightly cargo check --examples

test:
	rustup run nightly cargo test

debug-run:
	INSTA_API_SERVER_HOST=localhost:8077 rustup run nightly cargo run

run:
	rustup run nightly cargo run --release

build:
	rustup run nightly cargo build --release
