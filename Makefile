doc:
	rustup run nightly cargo doc
	docker run --rm -p 8042:80 -v `pwd`/target/doc:/usr/share/html nginx

run:
	rustup run nightly cargo run
