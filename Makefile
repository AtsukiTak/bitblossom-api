check:
	docker start -a bitblossom-check

test:
	docker start -a bitblossom-test

dev:
	docker run --rm -v `pwd`:/home/app bitblossom cargo run

stag:
	docker run -v `pwd`:/home/app bitblossom cargo run

init:
	docker build -t bitblossom .
	docker run --name bitblossom-check -v `pwd`:/home/app bitblossom bash -c "cargo check && cargo check --tests && cargo check --examples"
	docker run --name bitblossom-test -v `pwd`:/home/app bitblossom bash -c "cargo test"
