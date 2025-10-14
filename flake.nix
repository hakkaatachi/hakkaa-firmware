# This flake is largly based on the following example:
# https://github.com/SFrijters/nix-qemu-esp32c3-rust-example
# Thanks and credits goes to SFrijters.
{
  description = "hakkaa firmware flake";
  inputs = {
    qemu-espressif.url = "github:SFrijters/nix-qemu-espressif";
    nixpkgs.follows = "qemu-espressif/nixpkgs";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "qemu-espressif/nixpkgs";
    };
  };

  outputs = { nixpkgs, qemu-espressif, rust-overlay, ... }:
    let
      inherit (nixpkgs) lib;
      flatAttrs = [ "overlays" "nixosModules" ];
      # Inject a system attribute if the attribute is not one of the above
      injectSystem = system:
        lib.mapAttrs (name: value:
          if lib.elem name flatAttrs then value else { ${system} = value; });
      # Combine the above for a list of 'systems'
      forSystems = systems: f:
        lib.attrsets.foldlAttrs (acc: system: value:
          lib.attrsets.recursiveUpdate acc (injectSystem system value)) { }
        (lib.genAttrs systems f);

      # Maybe other systems work as well, but they have not been tested
    in forSystems [
      "x86_64-linux"
      "aarch64-linux"
      "x86_64-darwin"
      "aarch64-darwin"
    ] (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) (import qemu-espressif) ];
        };
        toolchain =
          (pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml);
        name = "hackaa-firmware";
        qemu-esp32c3 = pkgs.qemu-esp32c3;
      in {
        packages = { inherit qemu-esp32c3; };
        devShells.default = pkgs.mkShellNoCC {
          name = "${name}-dev";

          packages = [
            pkgs.espflash
            pkgs.esptool
            pkgs.gnugrep
            pkgs.netcat
            qemu-esp32c3
            toolchain
          ];
        };
        formatter = pkgs.nixfmt-tree;
      });
}
