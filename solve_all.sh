#!/usr/bin/env sh

# Run with --release because the parsing is very slow with debug optimizations.
cargo test --release
