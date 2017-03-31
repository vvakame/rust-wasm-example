#!/bin/sh -eux

rustc --target=wasm32-unknown-emscripten hello.rs
echo "open http://localhost:8989/"
python -m SimpleHTTPServer 8989
