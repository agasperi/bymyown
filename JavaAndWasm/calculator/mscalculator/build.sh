#!/bin/bash
cd ../lib
cargo clean
cargo build --target wasm32-wasi --release
cp -rf ./target/wasm32-wasi/release/calc.wasm ../mscalculator/src/main/resources/calc.wasm
cd ../mscalculator
mvn clean package
mvn spring-boot:run