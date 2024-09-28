{ pkgs ? import <nixpkgs> { } }:

with pkgs;

mkShell rec {
  nativeBuildInputs = [
    pkg-config
    openssl
  ];
  buildInputs = [
    xorg.libX11
    xorg.libXrandr
    xorg.libXcursor
    xorg.libXi
    libxkbcommon
    libGL
    fontconfig
    wayland
  ];
  LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;
}