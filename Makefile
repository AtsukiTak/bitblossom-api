build:
	docker build -t bitblossom-api .

build-debug:
	docker build -t bitblossom-api:debug -f Dockerfile-debug .

clean:
	-docker rmi `docker images -qf "dangling=true"` 2> /dev/null
	-docker rm -f `docker ps -aq` 2> /dev/null
