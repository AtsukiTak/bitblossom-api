check:
	docker start bitblossom-check

test:
	docker start bitblossom-test

dev:
	docker run --rm -v `pwd`:/home/app bitblossom cargo run

stag:
	docker run --rm -v `pwd`:/home/app bitblossom cargo run

init:
	docker build -t bitblossom .
	docker run --name bitblossom-check -v `pwd`:/home/app bitblossom ["cargo", "check", "&&", "cargo", "check", "--tests", "&&", "cargo", "check", "--examples"]
	docker run --name bitblossom-test -v `pwd`:/home/app bitblossom ["cargo", "test"]
