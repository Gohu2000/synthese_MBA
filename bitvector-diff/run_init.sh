#!/bin/bash

rustup install 1.87.0
rustup override set 1.87.0
cargo clean
cargo update
cargo build -r
./target/release/generate_instances