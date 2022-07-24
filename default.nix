let tools = import ./tools.nix; in with tools;

stdenv.mkDerivation {
    name = "web-compress";
    unpackPhase = ''
    cp -r ${./Cargo.toml} Cargo.toml
    cp -r ${./.cargo} .cargo
    cp -r ${./Trunk.toml} Trunk.toml
    cp -r ${./index.html} index.html
    cp -r ${./src} src
    '';
    buildInputs = [wasm-bindgen-cli trunk binaryen cargo];
    buildPhase = ''
    trunk build
    '';
    installPhase = ''
    cp -r site $out
    '';
}
