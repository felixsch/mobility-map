import L from 'leaflet'

export class AccessibleAreaLayer extends L.FeatureGroup {
  private _layers: L.Layer[] = [];
  private _distance: number  = 500;

  setDistance(distance: number): this {
    this._distance = distance;
    this.update();
    return this;
  }

  onAdd(map: L.Map): this {
    this.update();

    map.on('moveend', this.update, this);
    return this;
  }

  onRemove(map: L.Map): this {
    this.clearLayers();
    map.off('moveend', this.update, this);
    return this;
  }

  update(): void {
    let bounds: L.LatLngBounds = this._map.getBounds();
    let bbox: string = `${bounds.getWest()},${bounds.getSouth()},${bounds.getEast()},${bounds.getNorth()}`;

    fetch(`/api/accessible_area?distance=${this._distance}&bbox=${bbox}`)
    .then(response => response.json())
    .then((data: GeoJSON.GeoJsonObject) => {
      this.clearLayers()
      this.addData(data)
    })
  }

  private addData(data: GeoJSON.GeoJsonObject): void {
    const geoJsonLayer = L.geoJson(data);

    geoJsonLayer.addTo(this);
  }
}
