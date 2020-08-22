// import 'ol/ol.css';
// import GPX from 'ol/format/GPX';
// import {Vector as VectorLayer} from 'ol/layer';
// import VectorSource from 'ol/source/Vector';
// import {Circle as CircleStyle, Fill, Stroke, Style} from 'ol/style';


export async function read_gpx(gpx_url) {
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
    vector.set('name', 'latest');

    window.mymap.addLayer(vector);
    vector.getSource().once('change', function (evt) {
        const source = evt.target;
        if (source.getState() === 'ready') {
            const extent = source.getExtent();
            window.mymap.getView().fit(extent, window.mymap.getSize());
        }
    });

    return vector.get('name');
}

export async function read_kml(kml_url) {
    console.log("Printing kml" + kml_url)
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
            url: kml_url,
            format: new ol.format.KML()
        }),
        style: function(feature) {
            return style[feature.getGeometry().getType()];
        },
        useSpatialIndex: false
    });
    vector.set('name', 'latest');

    window.mymap.addLayer(vector);
    vector.getSource().once('change', function (evt) {
        const source = evt.target;
        if (source.getState() === 'ready') {
            const extent = source.getExtent();
            window.mymap.getView().fit(extent, window.mymap.getSize());
        }
    });

    return vector.get('name');
}

export function remove(id) {
    const layers = window.mymap.getLayers().getArray();
    for (const layer of layers) {
        let name = layer.get('name');
        console.log("Found layer: " + name);
        if (name === id) {
            window.mymap.removeLayer(layer);
        }
    }

}
