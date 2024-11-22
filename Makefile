# load possible .env file
ifneq (,$(wildcard ./.env))
    include .env
    export
endif

.PHONY: build psql clean confirm_clean import-osm-data import-gtfs-data migrate-db

build:
	docker-compose build x-mobility-map

psql:
	@echo "Connecting to postgis database..."
	docker-compose exec -ti postgis psql -U ${POSTGRES_USER}

confirm_clean:
	@echo -n "Are you sure? This wipes also volumes![y/N] " && read ans && [ $${ans:-N} = y ]

clean: confirm_clean
	@echo "Removing all data.."
	docker-compose down -v --remove-orphans

migrate-db:
	@echo "Migrating database.."
	docker-compose up -d migration postgis

import-osm-data: migrate-db
	@echo "Importing osm data.. This takes a while!"
	./scripts/import-osm-data

import-gtfsdata: migrate-db
	@echo "Importing gtfs data.. This takes a while!"
	docker-compose run --rm gtfs-importer run
	
