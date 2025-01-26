CREATE TABLE counties (
    id VARCHAR(255),
    households INT,
    stops INT,
    name VARCHAR(255)
);
ALTER TABLE counties ADD CONSTRAINT unique_counties UNIQUE (id);

CREATE FUNCTION county_by_asg(asg TEXT)
RETURNS TABLE(name TEXT, geom GEOMETRY) AS $$
BEGIN
    RETURN QUERY
    SELECT b.name, b.geom
    FROM osm_counties b
    WHERE b.asg = asg
    LIMIT 1;
END;
$$ LANGUAGE plpgsql;
