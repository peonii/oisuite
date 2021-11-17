#!/bin/sh

if [ ! -d "~/bin" ]; then
    mkdir ~/bin
fi
cargo build --release
cp -f target/release/oisuite ~/bin/

