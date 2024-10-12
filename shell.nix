{ pkgs ? import <nixpkgs> { }, lib ? pkgs.lib }:

pkgs.mkShell rec {
  nativeBuildInputs = with pkgs; [
    pkg-config
  ];

  buildInputs = with pkgs; [
    alsa-lib
    libGL
    udev
    vulkan-loader
    xorg.libX11
    xorg.libXcursor
    xorg.libXi
    xorg.libXrandr
  ];

  LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;
}
