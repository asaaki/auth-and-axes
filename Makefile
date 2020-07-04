PROJECT_NAME = auth-n-axes

export DOCKER_CLI_EXPERIMENTAL ?= enabled
export DOCKER_BUILDKIT ?= 1
export COMPOSE_DOCKER_CLI_BUILD ?= 1
export BUILDKIT_PROGRESS ?= plain

DOCKER = docker
BK_BUILD = $(DOCKER) build --progress=plain
DOCKER_NETWORK = k3d-$(NAME)

DC = docker-compose
DC_RUN = $(DC) run --rm
DC_UP = $(DC) up -d

build:
	docker-compose build --pull --force-rm api

start:
	docker-compose up -d

stop:
	docker-compose down --remove-orphans

restart: stop start

clean: stop
	docker-compose rm --force

logs:
	docker-compose logs

list-images:
	docker images --filter=reference='$(PROJECT_NAME)/*'

request:
	curl -i http://localhost:8085/api

request-login:
	curl -i -XPOST -d'{"email":"test1","password":"pw"}' https://id.unicorn.test:8099/login -H 'origin: https://app.unicorn.test:8081'

request-login-options:
	curl -i -XOPTIONS https://id.unicorn.test:8099/login -H 'origin: https://app.unicorn.test:8081'
