CREATE TABLE stop_stats (
    stop_id VARCHAR(255) NOT NULL,
    geom GEOMETRY(Geometry, 3857) NOT NULL,
    distance INT,
    total_residents INT,
    total_houses INT,
    total_flats INT,
    last_updated_at TIMESTAMP
);

ALTER TABLE stop_stats ADD CONSTRAINT fk_stop_stats_stop_id FOREIGN KEY (stop_id) REFERENCES stops(id);
ALTER TABLE stop_stats ADD CONSTRAINT unique_stop_stats UNIQUE (stop_id, distance);

CREATE OR REPLACE FUNCTION residential_buildings_near(point geometry(Point, 3857), distance numeric)
RETURNS TABLE(id integer, geom geometry(Geometry,3857), units integer, levels integer) AS $$
BEGIN
  RETURN QUERY
  SELECT b.id, b.geom, b.units, b.levels FROM osm_buildings b
    WHERE b.residential = TRUE
    AND ST_DWithin(point, b.geom, distance);
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION residential_buildings_hull(point geometry(Point, 3857), distance numeric)
RETURNS geometry(Geometry, 3857) AS $$
DECLARE
  hull geometry(Geometry, 3857);
BEGIN
  SELECT ST_ConvexHull(ST_Collect(geom))
  INTO hull
  FROM residential_buildings_near(point, distance);
  RETURN hull;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION residential_buildings_stats(point geometry(Point, 3857), distance numeric)
RETURNS TABLE(total_houses bigint, total_flats bigint, total_residents bigint) AS $$
BEGIN
  RETURN QUERY
  SELECT COUNT(*) AS total_houses,
         SUM(units * levels * 2) AS total_flats,
         SUM(units * levels * 2 * 2.1)::bigint AS total_residents
  FROM residential_buildings_near(point, distance);
END;
$$ LANGUAGE plpgsql;
