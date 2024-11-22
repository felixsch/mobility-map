CREATE EXTENSION IF NOT EXISTS postgis;

CREATE TABLE stop_times (
    arrival_time TIME,
    departure_time TIME,
    stop_id VARCHAR(255) NOT NULL,
    stop_sequence INTEGER NOT NULL,
    pickup_type INTEGER,
    drop_off_type INTEGER,
    stop_headsign VARCHAR(255)
);

CREATE TABLE stops (
    stop_id VARCHAR(255) PRIMARY KEY,
    stop_code VARCHAR(255),
    stop_name VARCHAR(255),
    stop_desc TEXT,
    location GEOMETRY(Point, 4326),
    parent_station VARCHAR(255),
    wheelchair_boarding INTEGER,
    platform_code VARCHAR(255),
    level INTEGER
);

CREATE INDEX idx_stop_times_stop_id ON stop_times (stop_id);
CREATE INDEX idx_stops_location ON stops USING GIST (location);
CREATE INDEX idx_stops_stop_code ON stops (stop_code);
