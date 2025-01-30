APPLICATION_NAME ?= rust-test-1

build:
	docker build -t ${APPLICATION_NAME} .

run:
	docker run --rm --network host -d ${APPLICATION_NAME}

bash:
	id=$$(docker ps | grep rust-test-1 | awk '{print $$1}');\
	docker exec -it $$id /bin/bash

kill:
	id=$$(docker ps | grep rust-test-1 | awk '{print $$1}');\
	docker kill $$id
