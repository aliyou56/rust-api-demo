{ pkgs ? import <nixpkgs> {}}:

pkgs.mkShell {
  buildInputs = with pkgs; [
    rustc
    cargo
    rustfmt
    rust-analyzer
    clippy

    pkg-config
    openssl
  ];

  RUST_BACKTRACE = 1;
}

