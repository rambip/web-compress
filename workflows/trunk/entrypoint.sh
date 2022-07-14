#!/bin/sh

echo "home is $HOME"

cp -r /root $HOME

cd /root
ls -la

cd $HOME
ls -la

rustup toolchain list

/root/.cargo/bin/trunk $@
