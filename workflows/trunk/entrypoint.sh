#!/bin/sh


export RUSTUP_HOME=/root/.rustup

rustup toolchain list

/root/.cargo/bin/trunk $@
