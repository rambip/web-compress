# to get the exact rust version I want, with wasm enabled
let rust-overlay = (import (builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz"));

    change-rust-toolchain = self: super:
rec {
    rust-custom = self.rust-bin.stable."${super.rustc.version}".minimal.override {
        targets = ["wasm32-unknown-unknown"];
    };
    rustc = rust-custom;
    cargo = rust-custom;
};

# custom version of wasm-bindgen
    change-wasm-bindgen = self: super:
rec {
# best build-tool for rust packages
    naersk = super.callPackage (import (builtins.fetchTarball "https://github.com/nix-community/naersk/archive/master.tar.gz")) {};
    wasm-bindgen-cli = naersk.buildPackage {
        src =(super.fetchCrate {
                pname = "wasm-bindgen-cli";
                version = "0.2.78";
                sha256 = "sha256-5s+HidnVfDV0AXA+/YcXNGVjv/E9JeK0Ttng4mCVX8M=";
                });
        buildInputs = [super.openssl];
        nativeBuildInputs = [ super.pkg-config ];
    };
};

in

import <nixpkgs> {overlays=[
    rust-overlay
    change-rust-toolchain
    change-wasm-bindgen
];}
