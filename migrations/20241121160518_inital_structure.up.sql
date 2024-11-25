CREATE EXTENSION IF NOT EXISTS postgis;

CREATE TABLE stop_times (
    id VARCHAR(255) PRIMARY KEY,
    arrival TIME,
    departure TIME
);

CREATE TABLE stops (
    id VARCHAR(255) PRIMARY KEY,
    name VARCHAR(255),
    location GEOMETRY(Point, 4326)
);

CREATE INDEX idx_stop_times_stop_id ON stop_times (id);
CREATE INDEX idx_stops_location ON stops USING GIST (location);
