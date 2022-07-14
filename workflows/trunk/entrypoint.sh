#!/bin/sh


# copy the cached directory from the github vm to the container
cp -r $HOME/.cargo/registry $HOME/.cargo
cp -r $HOME/.cache $HOME/.cache

export HOME=/root

rustup toolchain list

/root/.cargo/bin/trunk $@
