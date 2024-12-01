# load possible .env file
ifneq (,$(wildcard ./.env))
    include .env
    export
endif

.PHONY: build psql clean confirm_clean import-osm-data import-gtfs-data migrate-database run-frontend

build:
	docker-compose build x-mobility-map

shell:
	docker-compose run --rm -it -v $(CURDIR):/usr/src/mobility-map x-mobility-map

psql:
	@echo "Connecting to postgis database..."
	docker-compose exec -ti postgis psql -U ${POSTGRES_USER}

confirm_clean:
	@echo -n "Are you sure? This wipes also volumes![y/N] " && read ans && [ $${ans:-N} = y ]

clean: confirm_clean
	@echo "Removing all data.."
	docker-compose down -v --remove-orphans

migrate-database:
	@echo "Migrating database.."
	docker-compose run -it --rm db-migrator migrate-database

import-osm-data: migrate-database
	@echo "Importing osm data.. This takes a while!"
	docker-compose run --rm osm-importer       \
		--database=${DATABASE_URL}         \
		--create                           \
		--prefix osm                       \
		--output=flex                      \
		--style=/config/mobility-flex.lua  \
		${EXTRACT_FILE}

import-gtfs-data: migrate-database
	@echo "Importing gtfs data.. This takes a while!"
	docker-compose run --rm gtfs-importer import-gtfs-data

run-frontend: migrate-database
	@echo "Running frontend node"
	docker-compose run --rm frontend run-frontend
	
