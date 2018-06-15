build:
	docker build -t bitblossom-api .

clean:
	docker rm `docker ps -aq`
	docker rmi `docker images -qf "dangling=true"`
