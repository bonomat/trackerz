# Trackerz

[![Built With: RUST ❤️](https://img.shields.io/badge/Built%20With-RUST-lightgrey)](https://www.rust-lang.org/) [![WASM: Yew](https://img.shields.io/badge/WASM-Yew-brightgreen)](https://yew.rs/)


## Usage

Just run the `run` script; it should correctly initialize the environment at the first run (both cargo and npm)

```bash
./run
```

If this does not work, have a look at `run` and manually install the dependencies.

You will also need a `.env` file:

```bash
BASEMAP_KEY=XXX
```

You can retrieve the key from [Mapbox](https://www.mapbox.com/).

### Caveat: Cleanup

Sometimes the hot-reload server or the `thttp` server just stay up, while building fails, etc.
In case of "zombie" servers, try to fix it with `--clean`

```bash
./run --clean
```
