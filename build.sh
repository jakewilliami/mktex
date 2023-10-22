#!/usr/bin/env sh

set -xe
trap 'exit 1' INT

# Build programme
MODE="release"
cargo build --$MODE
D="$(basename "$PWD")"
F="${D%%.*}"
strip ./target/$MODE/$F
cp -f ./target/$MODE/$F ./

