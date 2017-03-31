#!/bin/sh -eux

rustc --target=wasm32-unknown-emscripten hello.rs -o hello.html
echo "open http://localhost:8989/hello.html"
python -m SimpleHTTPServer 8989
