let
  rust_overlay = import (builtins.fetchTarball https://github.com/oxalica/rust-overlay/archive/master.tar.gz);
  pkgs = import <nixpkgs> { overlays = [ rust_overlay ]; };
  rust-toolchain = pkgs.rust-bin.nightly."2023-07-01".default.override {
    extensions = [ "rust-src" ];
  };
in
with pkgs;
mkShell {
  buildInputs = [
    rust-toolchain
  ];
  RUST_SRC_PATH="${rust-toolchain}/lib/rustlib/src/rust/library/";
  RUST_BACKTRACE = 1;
}
