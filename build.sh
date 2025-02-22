#!/bin/bash

# Exit on any error
set -e

echo "Building BBL Web Demo..."

# Check if wasm-pack is installed
if ! command -v wasm-pack &> /dev/null; then
    echo "wasm-pack not found. Installing..."
    curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
fi

# Build the WebAssembly module
echo "Building WebAssembly module..."
wasm-pack build web --target web --out-dir web/wasm

# Start the Python server
echo "Starting Python server..."
cd web
python3 server.py
