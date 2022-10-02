#!/bin/env bash

orig_dir=$(dirname $0)
game_dir="../game"

build_sh="build.sh"

cd "$game_dir" || exit 1

eval "$build_sh"
cargo run

cd "$orig_dir" || exit 1
