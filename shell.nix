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
recent_trunk = with pkgs;
rustPlatform.buildRustPackage rec {
  pname = "trunk";
  version = "0.14.0";

  src = fetchFromGitHub {
    owner = "thedodd";
    repo = "trunk";
    rev = "v${version}";
    sha256 = "sha256-69MQDIF79pSuaOgZEIqb/ESPQzL7MUiQaJaxPccGxo8=";
  };

  nativeBuildInputs = [ pkg-config ];
  buildInputs = if stdenv.isDarwin
    then [ libiconv CoreServices Security ]
    else [ openssl ];

  # requires network
  checkFlags = [ "--skip=tools::tests::download_and_install_binaries" ];

  cargoSha256 = "sha256-3WTxCMNpBmiNbZMHp5BrqTXa1vmE/ZZ/8XbdcfxBfYg=";

  meta = with lib; {
    homepage = "https://github.com/thedodd/trunk";
    description = "Build, bundle & ship your Rust WASM application to the web";
    maintainers = with maintainers; [ freezeboy ];
    license = with licenses; [ asl20 ];
  };
};
in
  with pkgs;
  mkShell {
    #name = "rust-env";
    buildInputs = [
      recent_trunk
      rust
      wasm
    ];
}

