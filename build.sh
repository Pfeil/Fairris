#!/usr/bin/env bash

function exec() {
    echo "RUN CMD:" $@
    $@
}

echo "Usage: Any parameters will be given to wasm-pack."
echo "Release builds: Do not give any other parameter."
echo "Debug builds (faster, larger file size): bash build.sh --dev"

exec wasm-pack build --target web --out-name wasm $@ --out-dir ./static
