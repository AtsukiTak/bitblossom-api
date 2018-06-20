build:
	docker build -t bluumm-api:$(shell git rev-parse HEAD | cut -c-7) .

build-debug:
	docker build -t bluumm-api:$(shell git rev-parse HEAD | cut -c-7)-debug --build-arg DEBUG=1 .

clean:
	-docker rmi `docker images -qf "dangling=true"` 2> /dev/null
	-docker rm -f `docker ps -aq` 2> /dev/null
