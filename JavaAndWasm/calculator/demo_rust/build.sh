#!/bin/bash
cd ../lib
cargo clean
cargo build --target wasm32-wasi --release
cp -rf ./target/wasm32-wasi/release/calc.wasm ../demo_rust/calc.wasm
cd ../demo_rust
cargo clean
cargo build