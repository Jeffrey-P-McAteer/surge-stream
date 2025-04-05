
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

 - https://crates.io/crates/blue_engine
 - https://crates.io/crates/dotrix
 - https://github.com/FyroxEngine/Fyrox?tab=readme-ov-file / https://fyrox.rs/

 - https://github.com/coreylowman/cudarc
 - https://github.com/rayon-rs/rayon

 - https://www.arcgis.com/apps/View/index.html?appid=5e7f84d84b4542f09b17c398a90ec5be
 - https://services.arcgis.com/jDGuO8tYggdCCnUJ/arcgis/rest/services/PLALLPLS_polyline/FeatureServer/0

 - https://github.com/easbar/fast_paths
 - https://docs.rs/serde-pickle/latest/serde_pickle/
 -


