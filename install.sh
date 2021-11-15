#!/bin/sh

cargo build --release
cp -f target/release/oisuite ~/bin/

