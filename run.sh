#!/usr/bin/env bash

set -eo pipefail

# Run Program
cargo build --jobs 1
sudo ./target/debug/containify run
