#!/bin/env bash

orig_dir=$(pwd)
sh_dir=$(dirname $0)



# compile shaders
# ---------------
shader_file="game/shaders/sprite"
shader_sh="$sh_dir/../hellengine/tools/compile_shaders.sh"
eval "$shader_sh" "$shader_file"



# compile game
# ------------

game_dir="$sh_dir/../game"

cd "$game_dir" || exit 1

cargo build

cd "$orig_dir" || exit 1
