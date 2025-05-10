# Simple 3D Rasterizer

A flexible 3D rasterizer with multiple output interfaces. Currently supports WebAssembly (rendering to canvas) and command-line output.

![WASM Demo](demo/wasm.mkv) | ![CLI Demo](demo/cli.mkv)
---------------------------|--------------------------

## Features

- Load OBJ files (triangle faces only)
- Load image textures
- Rasterize triangles to screen
- Extensible interface system (implement your own render targets)
- Input handling (keys, timers, window resizing)
- Multiple output options
  - WebAssembly (canvas rendering)
  - Command-line ASCII output

## Getting Started

### Prerequisites

- Rust toolchain
- For WASM: `wasm-pack`
- For Nix builds: Nix package manager, direnv (optional)

### Running the Demos

#### Using Rust directly:
```bash
# CLI version
cd simple-3d-cli
cargo run

# WASM version
cd simple-3d-wasm
wasm-pack build --target web
# Then in another terminal
python -m http.server 9999
```

#### Using Nix and Direnv
```bash
# Command line version
bc

# WASM version
bw
# Then in another terminal
s
```

## Adding Custom Models

1. Place your `.obj` file in the `assets` folder
2. Ensure your model uses only triangles for faces
3. Modify the appropriate code to load your new asset

## Extending the Rasterizer

You can implement new output interfaces by creating a new implementation of the `Interface` trait. See existing implementations for reference.

## Project Structure

- `simple-3d-core/` - Core rasterizer logic
- `simple-3d-cli/` - Command line interface implementation
- `simple-3d-wasm/` - WebAssembly interface implementation
- `assets/` - Contains models and textures

## Contributing

Pull requests are welcome! For major changes, please open an issue first to discuss what you'd like to change.

## License

[MIT](https://choosealicense.com/licenses/mit/)
