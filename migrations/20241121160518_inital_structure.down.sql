DROP INDEX IF EXISTS idx_stop_times_stop_id;
DROP TABLE IF EXISTS stop_times CASCADE;

DROP INDEX IF EXISTS idx_stops_location;
DROP INDEX IF EXISTS idx_stops_stop_code;
DROP TABLE IF EXISTS stops;
