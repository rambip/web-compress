#!/bin/sh

echo "home is $HOME"

cp -r /root $HOME

rustup toolchain list

/root/.cargo/bin/trunk $@
