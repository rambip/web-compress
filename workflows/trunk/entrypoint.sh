#!/bin/sh

echo "home is $HOME"

cp -r /root/.rustup $HOME/.rustup
cp -r /root/.cargo $HOME/.cargo

rustup toolchain list

/root/.cargo/bin/trunk $@
