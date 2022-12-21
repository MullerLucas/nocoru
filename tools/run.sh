#!/bin/env bash

orig_dir=$(pwd)
sh_dir=$(dirname $0)
game_dir="$sh_dir/../game"
build_sh="$orig_dir/$sh_dir/build.sh"



# build game
# ----------
eval "$build_sh"



# run game
# --------
cd "$game_dir" || exit 1
cargo-clippy
cargo run
cd "$orig_dir" || exit 1
