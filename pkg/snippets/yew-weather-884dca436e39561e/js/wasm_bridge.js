export function update_map() {
    window.mymap.fire('submit');
}

export function read_gpx(gpx_url) {
    var runLayer = omnivore.gpx(gpx_url)
        .on('ready', function () {
            window.mymap.fitBounds(runLayer.getBounds());
        })
        .addTo(window.mymap);
    window.runLayer = runLayer;
    console.log(runLayer);
    console.log("Hallo");
}

export function remove() {
    window.runLayer.remove();
}
