#!/bin/env bash

POSITIONAL_ARGS=()

while [[ $# -gt 0 ]]; do
  case $1 in
    -c|--check)
      SHOULD_CHECK=YES
      shift # past value
      ;;
    -r|--run)
      SHOULD_RUN=YES
      shift # past argument
      ;;
    # --default)
    #   SEARCHPATH="$2"
    #   shift # past argument
    #   shift # past value
    #   ;;
    -*|--*)
      echo "Unknown option $1"
      exit 1
      ;;
    *)
      POSITIONAL_ARGS+=("$1") # save positional arg
      shift # past argument
      ;;
  esac
done


echo "SHOULD_CHECK: '$SHOULD_CHECK'"
echo "  SHOULD_RUN: '$SHOULD_RUN'"

# -----------------------------------------------


orig_dir=$(pwd)
sh_dir=$(dirname $0)


# compile shaders
# ---------------
sh_shader="$sh_dir/build_shaders.sh"
chmod +x "$sh_shader"
eval "$sh_shader"

# compile game
# ------------
game_dir="$sh_dir/.."

cd "$game_dir" || exit 1

# cargo-clippy

if [ "$SHOULD_CHECK" = "YES" ]; then
    echo "start checking project..."
    cargo check
elif [ "$SHOULD_RUN" = "YES" ]; then
    echo "start running project..."
    cargo run
else
    echo "start building project..."
    cargo build
fi




cd "$orig_dir" || exit 1
