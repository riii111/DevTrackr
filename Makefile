down:
	docker compose down -v

build:
	docker compose build

build-no-cache:
	docker compose build --no-cache

up:
	docker compose up -d

re:
	make down
	make build
	make up

re-n:
	make down
	make build-no-cache
	make up

container=frontend
command=/bin/bash
tail=200

logs:
	docker compose logs --tail=${tail} ${container}

logs-all:
	docker compose logs --tail=${tail}

exec:
	docker compose exec ${container} ${command}
