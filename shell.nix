let tools = import ./tools.nix; in

tools.pkgs.mkShell {
  nativeBuildInputs = with tools; [
    wasm-bindgen-cli
    trunk
    binaryen
    cargo
  ];
}
