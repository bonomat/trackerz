
const mymap = new ol.Map({
    target: 'map',
    layers: [
        new ol.layer.Tile({
            source: new ol.source.OSM()
        }),
    ],
    view: new ol.View({
        center: [0, 0],
        zoom: 4
    }),
    units: 'm',
});

export default mymap;
