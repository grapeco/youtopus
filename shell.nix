{ pkgs ? import <nixpkgs> {}, }:

with pkgs;

mkShell {
  buildInputs = with pkgs; [
    (pkgs.fenix.complete.withComponents [
      "cargo"
      "rustc"
      "rust-src"
    ])
    yt-dlp
    pkg-config
    openssl
  ];

  shellHook = ''
    export PATH=$PATH:''${CARGO_HOME:-~/.cargo}/bin
  '';
}
