{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    fenix.url = "github:nix-community/fenix";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    fenix,
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = nixpkgs.legacyPackages.${system};
    in {
      devShells.default = pkgs.mkShell {
        name = "devShell";

        buildInputs = with pkgs; [
          cargo
          rustc
          rust-analyzer

          pkg-config
          udev
          alsa-lib
          vulkan-loader
          xorg.libX11
          xorg.libXcursor
          xorg.libXi
          xorg.libXrandr
          libxkbcommon
          wayland
          glibc.dev
          glib.dev
        ];

        LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
          pkgs.vulkan-loader
          pkgs.libxkbcommon
          pkgs.wayland
          pkgs.alsa-lib
          pkgs.udev
          pkgs.xorg.libX11
          pkgs.xorg.libXcursor
          pkgs.xorg.libXi
          pkgs.xorg.libXrandr
        ];
      };
    });
}
