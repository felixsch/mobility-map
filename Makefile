# load possible .env file
ifneq (,$(wildcard ./.env))
    include .env
    export
endif

SOURCES := $(shell find . -name '*.rs')
MIGRATIONS := $(shell find ./migrations/ -name '*.sql')

.PHONY: psql clean database-up shell confirm_clean import-% run-%

build: $(SOURCES)
	@echo "BUILD"
	docker-compose build x-mobility-map

migrate: $(MIGRATIONS)
	@echo "MIGRATION"
	docker-compose run -it --rm db-migrator migrate-database

database-up:
	docker-compose up postgis -d


shell: build database-up migrate
	docker-compose run --rm -it -v $(CURDIR):/mobility-map -w /mobility-map x-mobility-map

psql:
	docker-compose exec -ti postgis psql -U ${POSTGRES_USER}

confirm_clean:
	@echo -n "Are you sure? This wipes also volumes![y/N] " && read ans && [ $${ans:-N} = y ]

clean: confirm_clean
	@echo "Removing all data.."
	docker-compose down -v --remove-orphans

import-%: build database-up migrate
	docker-compose run --rm -it importer import-$*

run-%: build database-up migrate
	docker-compose run --rm -it x-mobility-map run-$*

frontend: build database-up migrate
	docker-compose run --rm -it -p 3000:3000 x-mobility-map run-frontend
