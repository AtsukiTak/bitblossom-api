build:
	docker build -t bitblossom-api docker/release

build-debug:
	docker build -t bitblossom-api:debug docker/debug

clean:
	docker rmi `docker images -qf "dangling=true"`
	docker rm `docker ps -aq`
