# load possible .env file
ifneq (,$(wildcard ./.env))
    include .env
    export
endif

.PHONY: psql clean confirm_clean

psql:
	@echo "Connecting to PostgreSQL database..."
	docker-compose up -d
	docker-compose mobility-map-postgis exec -ti psql -u ${POSTGRES_PASSWORD}


confirm_clean:
	@echo -n "Are you sure? This wipes also volumes![y/N] " && read ans && [ $${ans:-N} = y ]

clean: confirm_clean
	@echo "Removing all data.."
	docker-compose down -v
