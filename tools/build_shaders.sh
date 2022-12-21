#!/bin/env bash

orig_dir=$(pwd)
sh_dir=$(dirname $0)

# compile shaders
# ---------------
shader_sh="$sh_dir/../hellengine/engine/tools/compile_shaders.sh"

input=(
    "$sh_dir/../game/shaders/sprite"
    "$sh_dir/../game/shaders/test"
    "$sh_dir/../game/shaders/bmfont"
)

echo "start compiling shaders..."

for in in "${input[@]}"; do
    echo "---> compiling shader '$in'..."
    eval "$shader_sh" "$in" || exit 1
done

echo "done compiling shaders..."
