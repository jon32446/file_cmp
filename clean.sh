#!/bin/bash
# Clean Rust build artifacts

set -e

echo "Cleaning Rust build artifacts..."
cargo clean
echo "Done."
