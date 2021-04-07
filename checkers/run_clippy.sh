#!/bin/sh

find . -name Cargo.toml -printf '%h\n'|xargs -P 2 -I {} sh -c "cd {} && cargo clippy --target=x86_64-unknown-linux-gnu -- -D clippy::pedantic -D clippy::all || exit 255"
