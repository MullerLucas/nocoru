#!/bin/env bash

orig_dir=$(dirname $0)
game_dir="../game"

cd "$game_dir" || exit 1

cargo-clippy check

cd "$orig_dir" || exit 1
