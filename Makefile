down:
	docker compose down

down-v:
	docker compose down -v

build:
	docker compose build

build-no-cache:
	docker compose build --no-cache

up:
	docker compose up -d

restart-all:
	make down -v
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
