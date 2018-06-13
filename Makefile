check:
	docker run --rm -v `pwd`:/home/app bitblossom cargo check
	docker run --rm -v `pwd`:/home/app bitblossom cargo check --tests
	docker run --rm -v `pwd`:/home/app bitblossom cargo check --examples

test:
	docker run --rm -v `pwd`:/home/app bitblossom cargo test

dev:
	docker run --rm -v `pwd`:/home/app bitblossom cargo run

stag:
	docker run --rm -v `pwd`:/home/app bitblossom cargo run

init:
	docker build -t bitblossom .
