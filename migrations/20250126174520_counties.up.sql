
CREATE OR REPLACE FUNCTION stops_within(geom geometry(Geometry, 3857))
RETURNS TABLE(id varchar) AS $$
BEGIN
  RETURN QUERY
    SELECT s.id FROM stops s WHERE ST_Within(s.location, geom);
END;
$$ LANGUAGE plpgsql;
