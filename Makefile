check:
	rustup run nightly cargo check
	rustup run nightly cargo check --tests
	rustup run nightly cargo check --examples

test:
	rustup run nightly cargo test

debug-build:
	rustup run nightly cargo build

build:
	rustup run nightly cargo build --release

clean:
	cargo clean
