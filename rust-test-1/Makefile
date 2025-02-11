APPLICATION_NAME ?= rust-test-1

build:
	cargo build

run:
	cargo run

d-build:
	docker build -t ${APPLICATION_NAME} .

d-run:
	docker run --rm --network host -d ${APPLICATION_NAME}

d-bash:
	id=$$(docker ps | grep rust-test-1 | awk '{print $$1}');\
	docker exec -it $$id /bin/bash

d-kill:
	id=$$(docker ps | grep rust-test-1 | awk '{print $$1}');\
	docker kill $$id
