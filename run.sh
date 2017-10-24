#!/bin/bash
export PATH=$PATH:$(pwd)
source $HOME/.profile
export DISPLAY=:0
cargo build
Xvfb &