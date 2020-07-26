// import 'ol/ol.css';
// import GPX from 'ol/format/GPX';
// import {Vector as VectorLayer} from 'ol/layer';
// import VectorSource from 'ol/source/Vector';
// import {Circle as CircleStyle, Fill, Stroke, Style} from 'ol/style';

export function read_gpx(gpx_url) {
    const style = {
        'Point': new ol.style.Style({
            image: new ol.style.Circle({
                fill: new ol.style.Fill({
                    color: 'rgba(255,255,0,0.4)'
                }),
                radius: 5,
                stroke: new ol.style.Stroke({
                    color: '#ff0',
                    width: 1
                })
            })
        }),
        'LineString': new ol.style.Style({
            stroke: new ol.style.Stroke({
                color: '#f00',
                width: 3
            })
        }),
        'MultiLineString': new ol.style.Style({
            stroke: new ol.style.Stroke({
                color: '#bb00ff',
                width: 3
            })
        })
    };

    const vector = new ol.layer.Vector ({
        source: new ol.source.Vector({
            url: gpx_url,
            format: new ol.format.GPX()
        }),
        style: function(feature) {
            return style[feature.getGeometry().getType()];
        },
        useSpatialIndex: false
    });
    window.mymap.addLayer(vector);
    vector.getSource().on('change', function (evt) {
        const source = evt.target;
        if (source.getState() === 'ready') {
            const numFeatures = source.getFeatures().length;
            console.log("Count after change: " + numFeatures);
            const extent = source.getExtent();
            console.log(extent);
            window.mymap.getView().fit(extent, window.mymap.getSize());
        }

    });
    // Get the array of features
    return vector;
}

export function remove(vector) {
    console.log("Received again: " + vector);
    window.mymap.removeLayer(vector);
}
