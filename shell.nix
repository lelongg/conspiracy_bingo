let
  rust-overlay = builtins.fetchTarball {
    url =
      "https://github.com/oxalica/rust-overlay/tarball/611e6213c5563a3f46a57c600c70e0f0fd2811f3";
    sha256 = "sha256:1z9yv2wcxpzf7y4lsv21lrvzwcvsfpgfjqsg53m5z3h5pdvap26g";
  };
  pkgs = import <nixpkgs> { overlays = [ (import (rust-overlay)) ]; };
in with pkgs;
let
  rustStable = (rustChannelOf { channel = "1.51.0"; }).rust.override {
    extensions = [ "rust-src" ];
    targets = [ "wasm32-unknown-unknown" ];
  };
in mkShell {
  buildInputs = [ just clang rustStable openssl pkgconfig wasm-pack miniserve ];
  LIBCLANG_PATH = "${llvmPackages.libclang}/lib";
}
