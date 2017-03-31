#!/bin/sh -eux

rustc -V
emcc -v

cargo fmt
cargo test
# cargo build --features "clippy"
# cargo clean
cargo build --target wasm32-unknown-emscripten

rm -rf dist/
mkdir dist/
cp static/*.html dist/
cp target/wasm32-unknown-emscripten/debug/deps/*.wasm dist/lib.wasm
# exclude .asm.js
cp target/wasm32-unknown-emscripten/debug/deps/*[0-9a-f].js dist/lib.js

echo "open http://localhost:8989/"
cd dist/ && python -m SimpleHTTPServer 8080

# cargo build --target x86_64-apple-darwin
# RUST_BACKTRACE=1 ./target/x86_64-apple-darwin/debug/helloworld-with-cargo 1 4

# If you want to run `fn main()`, try `Module.callMain(["1", "4"])` from DeveloperTools.
