-- Add down migration script here

DROP INDEX IF EXISTS unique_counties;
DROP TABLE IF EXISTS counties;

DROP FUNCTION IF EXISTS county_by_asg;
