build:
	docker build -t bitblossom-api .

build-debug:
	docker build -t bitblossom-api:debug -f Dockerfile-debug .

clean:
	docker rmi `docker images -qf "dangling=true"`
	docker rm `docker ps -aq`
