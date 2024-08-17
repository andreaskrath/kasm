{pkgs ? import <nixpkgs> {overlays = [(import <rust-overlay>)];}}: let
  rust = pkgs.rust-bin.stable.latest.default.override {
    extensions = ["rust-src"];
  };
in
  pkgs.mkShell {
    buildInputs = with pkgs; [
      rust # rust toolchain
      cargo-edit # utility for upgrading toml crates
      just # justfiles runner
    ];
  }
