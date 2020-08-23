
const sydneyLonLat = [151.2093,-33.8688];
const startLocation = ol.proj.fromLonLat(sydneyLonLat);
const mymap = new ol.Map({
    target: 'map',
    layers: [
        new ol.layer.Tile({
            source: new ol.source.OSM()
        }),
    ],
    view: new ol.View({
        center: startLocation,
        zoom: 8
    }),
    units: 'm',
});

export default mymap;
