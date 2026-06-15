#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/../examples"

cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-targets --all-features
