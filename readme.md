
# Surge-Stream

__TOC__

`surge-stream` is a command-line and GUI tool for modeling asset flows across a GIS network of features.

It supports performing this analysis on both Windows and Linux systems.

## Data Inputs

TODO

## Output Formats

TODO

## Development Dependencies

TODO

## Runtime Dependencies

TODO

## Running the Application

```bash
rm -rf ./data/ # If you want to delete old data + have a clean/fresh run
uv run fetch_data.py && cargo run --release data/raw-layer-data.pickle

# Misc one-liners
cargo run --release --target=x86_64-pc-windows-gnu ./data/raw-layer-data.pickle ./data/assumptions.toml
VERBOSE=1 cargo run --release --target=x86_64-unknown-linux-gnu ./data/raw-layer-data.pickle ./data/assumptions.toml

```

## Building the Application

Binaries are located under `./target/<host-triple>/release/surge-stream[.exe]`

From a Linux machine w/ deps installed, execute
```bash
uv run cross_compile_using_arch_container.py
```

From a Windows machine w/ deps installed, execute

```bash
cargo build --release
```

# Research

Also see the folder [./research-journey](./research-journey)

 - https://crates.io/crates/blue_engine
 - https://crates.io/crates/dotrix
 - https://github.com/FyroxEngine/Fyrox?tab=readme-ov-file / https://fyrox.rs/

 - https://github.com/coreylowman/cudarc
 - https://github.com/rayon-rs/rayon

 - https://www.arcgis.com/apps/View/index.html?appid=5e7f84d84b4542f09b17c398a90ec5be
 - https://services.arcgis.com/jDGuO8tYggdCCnUJ/arcgis/rest/services/PLALLPLS_polyline/FeatureServer/0

 - https://github.com/easbar/fast_paths
 - https://docs.rs/serde-pickle/latest/serde_pickle/
 - https://github.com/cjriley9/gpkg-rs


