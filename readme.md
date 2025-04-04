
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


