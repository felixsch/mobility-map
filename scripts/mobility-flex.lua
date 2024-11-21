local tables = {}

tables.osm_stops = osm2pgsql.define_node_table('osm_stops', {
    { column = 'stop_type', type = 'text' },
    { column = 'ifopt', type = 'text' },
    { column = 'tags', type = 'jsonb' },
    { column = 'geom', type = 'point', projection = 4326 },
})

tables.osm_residential_buildings = osm2pgsql.define_way_table('osm_residential_buildings', {
    { column = 'tags', type = 'jsonb' },
    { column = 'geom', type = 'geometry', projection = 4326 },
})

-- Determine the stop type of a node to store it as a stop
-- and also define which type of transportation media is used
-- e.g. bus or train or subway
local function determine_stop_type(tags)
    if tags.highway == 'bus_stop' then
      return 'bus'
    end

    if tags.railway == 'halt' or
       tags.railway == 'station' then

       if tags.subway == 'yes' or
          tags.station == 'subway' then
         return 'subway'
       end
       return 'train'
    end

    if tags.railway == 'tram_stop' then
      return 'tram'
    end

    return nil
end

local function is_residential(object)
    local area = object.area_tags or {}
    local residential_types = {
        residential = true,
        apartments = true,
        house = true,
        detached = true,
        dormitory = true,
        terrace = true,
        bungalow = true,
        static_caravan = true,
    }
    return residential_types[object.tags.building] ~= nil or area.landuse == 'residential'
end

function osm2pgsql.process_node(object)
    local type_of_stop = determine_stop_type(object.tags)
    print("[node] stop type: ", type_of_stop)

    if type_of_stop then
        print(" ==> insert")
        tables.osm_stops:insert({
            stop_type = type_of_stop,
            ifopt = object.tags.ifopt, -- Add the IFOPT information if available
            tags = osm2pgsql.tags_as_jsonb(object.tags),
            geom = object:as_point()
        })
    end
end

function osm2pgsql.process_way(object)
    if object.tags.building then
      print("[way] building: ", object.tags.building)
    end
    if object.tags.building and is_residential(object) then
        print(" ==> insert")
        tables.osm_residential_buildings:insert({
            tags = osm2pgsql.tags_as_jsonb(object.tags),
            geom = object:as_polygon(),
        })
    end
end

return tables
