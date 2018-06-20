clean:
	-docker rmi `docker images -qf "dangling=true"` 2> /dev/null
	-docker rm -f `docker ps -aq` 2> /dev/null
