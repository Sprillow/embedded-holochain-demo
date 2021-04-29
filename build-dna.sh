#!/bin/bash

cd dna/sample
cargo build --release --target wasm32-unknown-unknown
hc dna pack .
cd ../..