# load possible .env file
ifneq (,$(wildcard ./.env))
    include .env
    export
endif

.PHONY: psql clean confirm_clean import-osm

psql:
	@echo "Connecting to postgis database..."
	docker-compose up -d
	docker-compose exec -ti postgis psql -U ${POSTGRES_USER}

confirm_clean:
	@echo -n "Are you sure? This wipes also volumes![y/N] " && read ans && [ $${ans:-N} = y ]

clean: confirm_clean
	@echo "Removing all data.."
	docker-compose down -v --remove-orphans

import-osm-data:
	@echo "Importing osm data.. This takes a while!"
	docker-compose up -d migration postgis
	./scripts/import-osm-data init

update-osm-data:
	@echo "Update osm data.. This takes a while!"
	docker-compose up -d migration postgis
	./scripts/import-osm-data update

import-gtfs:
	@echo "Importing gtfs data.. This takes a while!"
	docker-compose up -d migration postgis
	docker-compose run --rm gtfs-import
	
