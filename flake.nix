{
  description = "A simple CHIP-8 emulator Rust project with GUI support";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        rustToolchain = pkgs.rust-bin.stable.latest.default;

        buildInputs = with pkgs; [
            lldb

            cargo
            cargo-edit

            openssl

            expat
            fontconfig
            freetype
            freetype.dev
            libGL
            pkg-config
            xorg.libX11
            xorg.libXcursor
            xorg.libXi
            xorg.libXrandr
            wayland
            libxkbcommon
          ];
      in
      {
        devShells.default = pkgs.mkShell {
          LD_LIBRARY_PATH =
            builtins.foldl' (a: b: "${a}:${b}/lib") "${pkgs.vulkan-loader}/lib" buildInputs;

          buildInputs = buildInputs ++ [
            rustToolchain
          ];
          shellHook = ''
            export RUST_BACKTRACE=1
          '';
        };
      }
    );
}
