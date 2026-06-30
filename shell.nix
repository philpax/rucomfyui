{ pkgs ? import <nixpkgs> { } }:

with pkgs;

mkShell rec {
  buildInputs = [
    libX11
    libXrandr
    libXcursor
    libXi
    libxkbcommon
    libGL
    vulkan-loader
    fontconfig
    wayland
  ];
  LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;
}
