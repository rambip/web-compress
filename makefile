site: src  
	docker run \
		-v ./:/project:z \
	    -v cache:/root/.cache \
	    -v cargo:/root/.cargo \
		trunk:latest build

container: Dockerfile
	docker build . -t trunk
	docker volume prune -f
	#docker volume create cache
	#docker volume create cargo

serve:
	docker run \
		-v ./:/project:z \
	    -v cache:/root/.cache \
	    -v cargo:/root/.cargo \
		-p 127.0.0.1:8080:8080 \
		trunk:latest serve --public-url=/ --address=0.0.0.0

clean:
	docker run \
		-v ./:/project:z \
		trunk:latest clean
