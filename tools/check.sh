#!/bin/env bash

orig_dir=$(pwd)
sh_dir=$(dirname $0)



# compile shaders
# ---------------
sprite_shader_file="$sh_dir/../game/shaders/sprite"
bmfont_shader_file="$sh_dir/../game/shaders/bmfont"
shader_sh="$sh_dir/../hellengine/engine/tools/compile_shaders.sh"

eval "$shader_sh" "$sprite_shader_file" || exit 1
eval "$shader_sh" "$bmfont_shader_file" || exit 1



# check game
# ----------
game_dir="$sh_dir/../game"
cd "$game_dir" || exit 1

cargo-clippy
# cargo check

cd "$orig_dir" || exit 1
