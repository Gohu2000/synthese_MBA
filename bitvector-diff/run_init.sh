#!/bin/bash

export CARGO_HOME=$HOME/.cargo
export RUSTUP_HOME=$HOME/.rustup
source $HOME/.cargo/env

rustup install 1.87.0
rustup override set 1.87.0
cargo clean
cargo update
cargo build -r
./target/release/generate_instances