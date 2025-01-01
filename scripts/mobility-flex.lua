local srid = 3857

local tables = {}

tables.osm_counties = osm2pgsql.define_relation_table('osm_counties', {
  { column = 'tags', type = 'jsonb' },
  { column = 'ags', type = 'text'},
  { column = 'name', type = 'text'},
  { column = 'geom', type = 'geometry', projection = srid, not_null = true },
})

tables.osm_states = osm2pgsql.define_relation_table('osm_states', {
  { column = 'tags', type = 'jsonb' },
  { column = 'ags', type = 'text'},
  { column = 'name', type = 'text'},
  { column = 'geom', type = 'geometry', projection = srid, not_null = true },
})

tables.osm_buildings = osm2pgsql.define_way_table('osm_buildings', {
  { column = 'id', sql_type = 'serial', create_only = true },
  { column = 'tags', type = 'jsonb' },
  { column = 'geom', type = 'geometry', projection = srid },
  { column = 'center', type = 'point', projection = srid },
  { column = 'units', type = 'int' },
  { column = 'levels', type = 'int' },
  { column = 'residential', type = 'bool' },
})

tables.osm_residental_areas = osm2pgsql.define_way_table('osm_residential_areas', {
  { column = 'tags', type = 'jsonb' },
  { column = 'geom', type = 'geometry', projection = srid, not_null = true },
})

local address_cache = {}

local function is_residential_building(object)
  -- see: https://wiki.openstreetmap.org/wiki/Key:building?uselang=en
  local residential_types = {
    apartments = true,
    barracks = true,
    house = true,
    detached = true,
    semidetached_house = true,
    dormitory = true,
    terrace = true,
    bungalow = true,
    static_caravan = true,
    residential = true,
  }
  return residential_types[object.tags.building] ~= nil
end


function estimate_levels(object)
  local levels = 1

  if object.tags['building:levels'] then
    levels = tonumber(object.tags['building:levels']) or 1
  end

  if object.tags['roof:levels'] then
    levels = levels + (tonumber(object.tags['roof:levels']) or 0)
  end

  return levels
end

-- There are multiple houses availale which have are mapped as one building
-- but are counted as multiple houses in for example the destatis database
-- To more accurately map those houses, detect their house numbers as multiple
-- to count them accordingly
-- example: https://www.openstreetmap.org/way/98740315
function estimate_units(object)
  local units = 0

  for _, id in ipairs(object.nodes) do
    if address_cache[id] ~= nil then
      units = units + 1
    end
  end

  return (units == 0) and 1 or units
end


function osm2pgsql.process_node(object)
  if object.tags['addr:housenumber'] then
    address_cache[object.id] = true
  end
end

function osm2pgsql.process_way(object)

  if object.is_closed == false then
    return
  end

  if object.tags.landuse == 'residential' then
    tables.osm_residental_areas:insert({
      tags = object.tags,
      geom = object:as_polygon(),
    })
  end

  if object.tags.building then
    res = is_residential_building(object)

    tables.osm_buildings:insert({
      tags = object.tags,
      geom = object:as_polygon(),
      center = object:as_polygon():centroid(),
      units = estimate_units(object),
      levels = estimate_levels(object),
      residential = is_residential_building(object)
,
    })
  end
end

function osm2pgsql.process_relation(object)
  if object.tags.boundary == 'administrative' then
    local admin_level = object.tags.admin_level

    if admin_level == '4' then
      tables.osm_states:insert({
        tags = object.tags,
        ags = object.tags['de:amtlicher_gemeindeschluessel'],
        name = object.tags.name,
        geom = object:as_multipolygon()
      })
    end

    if admin_level == '6' then
      tables.osm_counties:insert({
        tags = object.tags,
        ags = object.tags['de:amtlicher_gemeindeschluessel'],
        name = object.tags.name,
        geom = object:as_multipolygon()
      })
    end
  end
end
