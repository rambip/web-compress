let tools = import ./tools.nix; in with tools;

pkgs.stdenv.mkDerivation {
    name = "web-compress";
    src = ./.;
    buildInputs = [wasm-bindgen-cli trunk binaryen cargo];
    buildPhase = ''
    trunk build
    '';
    installPhase = ''
    cp -r site $out
    '';
}
