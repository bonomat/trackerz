
(function(l, r) { if (l.getElementById('livereloadscript')) return; r = l.createElement('script'); r.async = 1; r.src = '//' + (window.location.host || 'localhost').split(':')[0] + ':35729/livereload.js?snipver=1'; r.id = 'livereloadscript'; l.getElementsByTagName('head')[0].appendChild(r) })(window.document);
(function () {
    'use strict';

    let wasm;

    const heap = new Array(32).fill(undefined);

    heap.push(undefined, null, true, false);

    let heap_next = heap.length;

    let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });

    cachedTextDecoder.decode();

    let cachedTextEncoder = new TextEncoder('utf-8');

    /**
    */
    function run_app() {
        wasm.run_app();
    }

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

    async function main() {
      // await init("./pkg/southern_crossing_bg.wasm");
      run_app();
    }

    // Export the Leaflet map
    window.mymap = mymap;

    main();

}());
