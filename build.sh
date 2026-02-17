#!/usr/bin/env bash

# cargo run --release -- -t -l "örebro" -f 10
# cargo run --release -- -l "örebro" -f 10
# cargo run --release -- help
cargo build --release
/Users/simondanielsson/dev/rust/regn/target/release/regn -t

