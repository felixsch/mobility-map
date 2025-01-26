CREATE EXTENSION IF NOT EXISTS postgis;

CREATE TABLE stop_times (
    trip_id VARCHAR(255),
    stop_id VARCHAR(255),
    arrival TIME,
    departure TIME
);
ALTER TABLE stop_times ADD CONSTRAINT unique_stop_times UNIQUE (trip_id, stop_id, arrival, departure);

CREATE TABLE stops (
    id VARCHAR(255) PRIMARY KEY,
    name VARCHAR(255),
    location GEOMETRY(Point, 3857)
);

CREATE INDEX idx_stop_times_stop_id ON stop_times (stop_id);
CREATE INDEX idx_stops_location ON stops USING GIST (location);
CREATE INDEX idx_stop_id ON stops (id);
