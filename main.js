import init, { run_app } from "./pkg/southern_crossing.js";
import "./js/wasm_bridge.js";
import mymap from "./js/open_layer.js";

async function main() {
  await init("./pkg/southern_crossing_bg.wasm");
  run_app();
}

// Export the Leaflet map
window.mymap = mymap;

main();

