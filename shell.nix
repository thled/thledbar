let
  pkgs = import (
    fetchTarball("https://github.com/NixOS/nixpkgs/archive/ce6aa13369b667ac2542593170993504932eb836.tar.gz")
  ) {};
in pkgs.mkShell {
  buildInputs = with pkgs; [
    cargo
    rustc
    rustfmt
    clippy

    alsaLib
    cairo
    glib
    gobject-introspection
    openssl
    pango
    pkg-config
    python3
    xorg.libxcb
    xorg.xcbutilwm
  ];
}
