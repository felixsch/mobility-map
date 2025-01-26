ALTER TABLE stops ADD avg_cycle INT;

CREATE OR REPLACE FUNCTION average_cycle_time(ifopt TEXT)
RETURNS NUMERIC AS $$
DECLARE
  avg NUMERIC;
BEGIN
  SELECT TRUNC(EXTRACT(EPOCH FROM AVG(next - current)) / 60)
  INTO avg
  FROM (
    SELECT departure AS current,
           LEAD(departure) OVER (ORDER BY departure) AS next
    FROM stop_times
      WHERE stop_id = ifopt
      AND departure BETWEEN '08:00:00' AND '22:00:00'
  )
  WHERE next IS NOT NULL;
  RETURN avg;
END;
$$ LANGUAGE plpgsql;
