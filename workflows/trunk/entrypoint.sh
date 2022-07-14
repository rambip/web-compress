#!/bin/sh

export RUSTUP_HOME=/root/.rustup

du -h $HOME/.cache
du -h $HOME/.cargo

/root/.cargo/bin/trunk $@

du -h $HOME/.cache
du -h $HOME/.cargo
