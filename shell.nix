{ pkgs ? import <nixpkgs> { overlays = [ (import <rust-overlay>) ]; } }:
let
  rust = pkgs.rust-bin.stable.latest.default.override {
    extensions = [ "rust-src" ];
  };
in
pkgs.mkShell {
  buildInputs = [
    rust
  ];
}
