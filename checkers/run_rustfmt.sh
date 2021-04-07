#!/bin/sh

find . -name Cargo.toml -printf '%h\n'|xargs -P 2 -I {} sh -c "cd {} && cargo fmt -- --check || exit 255"
