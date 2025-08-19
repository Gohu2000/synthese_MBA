#!/bin/bash

# Install Rust/Cargo if not already present
if [ ! -d "$HOME/.cargo" ]; then
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
fi

# Add Cargo to the PATH for this session
source $HOME/.cargo/env

# Now, proceed with your original commands
cd ~/synthese_MBA/bitvector-diff
cargo build -r
./target/release/generate_instances