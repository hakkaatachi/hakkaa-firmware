# This flake is largly based on the following example:
# https://github.com/SFrijters/nix-qemu-esp32c3-rust-example
# Thanks and credits goes to SFrijters.
{
  description = "hakkaa firmware flake";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-25.05";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };
        toolchain =
          (pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml);
      in {
        devShells.default = pkgs.mkShellNoCC {
          name = "hakkaa-firmware";

          packages = [ pkgs.espflash pkgs.esptool toolchain ];
        };
        formatter = pkgs.nixfmt-tree;
      });
}
