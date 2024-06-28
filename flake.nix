{
  description = "Optimization-based Robot Motion Generation Method";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.05";
    utils.url = "github:numtide/flake-utils";
    devshell.url = "github:numtide/devshell";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, utils, devshell, rust-overlay, ... }@inputs:
    utils.lib.eachDefaultSystem (system: {
      devShells.default =
        let pkgs = import nixpkgs {
              inherit system;

              overlays = [
                devshell.overlays.default
                rust-overlay.overlays.default
              ];
            };

            toolchain = pkgs.rust-bin.fromRustupToolchainFile ./toolchain.toml;

        in pkgs.devshell.mkShell {
          name = "relaxed_ik_core";

          packages = [
            toolchain
            pkgs.rust-analyzer-unwrapped
          ];
        };
    });
}
