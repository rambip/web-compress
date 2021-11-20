{ pkgs ? import <nixpkgs> {} }:
  let
  moz_overlay = import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz);
  pkgs = import <nixpkgs> { overlays = [ moz_overlay ]; };
  channel = pkgs.rustChannelOf {
     date = "2021-11-01";
     channel = "nightly";
  };
  rust = (channel.rust.override {
    targets = [ "wasm32-unknown-unknown" ];
    extensions = ["rust-src" "rust-analysis"];
  });
  pkgsUnstable =
    import (fetchTarball https://github.com/NixOS/nixpkgs/archive/nixos-unstable.tar.gz) {};
in
  with pkgs;
  stdenv.mkDerivation {
    name = "rust-env";
    buildInputs = [
      pkgsUnstable.trunk
      rust
      wasm
    ];
}
