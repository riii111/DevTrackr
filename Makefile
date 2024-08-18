down:
	docker compose down

build:
	docker compose build

build-no-cache:
	docker compose build --no-cache

up:
	docker compose up

up-d:
	docker compose up -d

restart-all:
	make down
	make up-d

container=frontend
command=/bin/bash
tail=200

logs:
	docker compose logs --tail=${tail} ${container}

logs-all:
	docker compose logs --tail=${tail}

exec:
	docker compose exec ${container} ${command}
