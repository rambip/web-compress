#!/bin/sh

export HOME=/root

to_cache="
.cargo/registry/index
.cargo/registry/cache
.cargo/git
.cargo/.crates.toml
.cargo/.crates2.json
.cache
"

for dir in $to_cache
do
    mkdir -p $CACHE/$dir
    rm -rf $HOME/$dir
    ln -s $CACHE/$dir $HOME/$dir
done

cd $PROJECT
export CARGO_TARGET_DIR=$CACHE/target

/root/.cargo/bin/trunk $@
