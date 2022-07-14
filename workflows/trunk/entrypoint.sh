#!/bin/sh


export RUSTUP_HOME=/root

rustup toolchain list

/root/.cargo/bin/trunk $@
