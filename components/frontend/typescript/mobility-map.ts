import 'bootstrap/dist/css/bootstrap.min.css'
import 'leaflet/dist/leaflet.css'
import '../scss/mobility-map.scss'

import L from 'leaflet'
import {AccessibleAreaLayer} from './accessible_area'


const map = L.map('mobility-map')
const access_map = new AccessibleAreaLayer();
const attribution = '&copy; <a href="https://stadiamaps.com/" target="_blank">Stadia Maps</a>, &copy; <a href="https://openmaptiles.org/" target="_blank">OpenMapTiles</a> &copy; <a href="https://www.openstreetmap.org/copyright" target="_blank">OpenStreetMap</a>'

map.setView([49.4521, 11.0767], 13)

L.tileLayer('https://tiles.stadiamaps.com/tiles/alidade_smooth/{z}/{x}/{y}{r}.png', {
  maxZoom: 20,
  attribution: attribution,
}).addTo(map)

access_map.addTo(map)




function update_accessible_area() {
      // map.eachLayer((layer) => {
      //   if(layer.layer_type == 'accessible_area') {
      //     map.removeLayer(layer)
      //   }
      // });
      //
      // fetch(`/api/accessible_area`).then( async (response) => {
      //   L.geoJSON(await response.json(), {
      //     onEachFeature: async (_, layer) => {
      //       layer.
      //     }
      //   })
      //     .addTo(map)
      // });
}
