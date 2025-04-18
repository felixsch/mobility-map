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
	docker-compose run -it --rm mobility-map migrate

database-up:
	docker-compose up postgis -d

shell: build database-up migrate
	docker-compose run --rm -it --entrypoint /bin/bash -v $(CURDIR):/src run

psql:
	docker-compose exec -ti postgis psql -U ${POSTGRES_USER}

confirm_clean:
	@echo -n "Are you sure? This wipes also volumes![y/N] " && read ans && [ $${ans:-N} = y ]

clean: confirm_clean
	@echo "Removing all data.."
	docker-compose down -v --remove-orphans
