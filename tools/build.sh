#!/bin/bash

set -e
script=$(readlink -f "$0")
route=$(dirname "$script")

# cd ${route}/..
# colcon build --cmake-args -DCMAKE_BUILD_TYPE=Release -DCMAKE_EXPORT_COMPILE_COMMANDS=ON

export RUSTFLAGS="-A non_snake_case -A non_camel_case_types -A non_upper_case_globals -A dead_code -A unused_variables -A unused_assignments -A unused_imports"
cargo build --release --target=x86_64-unknown-linux-gnu