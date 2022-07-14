#!/bin/sh


export RUST_HOME=/root

rustup toolchain list

/root/.cargo/bin/trunk $@
