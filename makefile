site: src  
	docker run \
		-v ./:/project:z \
	    -v cache:/cache \
		trunk:latest build

container: workflows
	docker build workflows/trunk -t trunk

serve:
	docker run \
		-v ./:/project:ro \
	    -v cache:/cache \
		-p 127.0.0.1:8080:8080 \
		trunk:latest serve --public-url=/ --address=0.0.0.0

clean:
	docker run \
		-v ./:/project:z \
		trunk:latest clean
