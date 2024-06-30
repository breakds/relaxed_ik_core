{
  description = ''
    Implementation of our ICRA'23 paper: RangedIK: An Optimization-Based Robot
    Motion Generation Method for Ranged-Goal Tasks
  '';

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.05";
    utils.url = "github:numtide/flake-utils";
    devshell.url = "github:numtide/devshell";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, utils, devshell, rust-overlay, ... }@inputs: {
    overlays.default = final: prev: {
      pythonPackagesExtensions = prev.pythonPackagesExtensions ++ [
        (py-final: py-prev: {
          relaxed-ik = py-final.callPackage ./nix/pkgs/self {};
        })
      ];
    };
  } // utils.lib.eachDefaultSystem (system: {
    # The main development environment
    devShells.default =
      let pkgs = import nixpkgs {
            inherit system;

            overlays = [
              devshell.overlays.default
              rust-overlay.overlays.default
            ];
          };

          toolchain = pkgs.rust-bin.fromRustupToolchainFile ./toolchain.toml;

          pythonEnv = pkgs.python3.withPackages (ps: with ps; [
            numpy
          ]);

      in pkgs.devshell.mkShell {
        name = "relaxed-ik";

        commands = with pkgs; [
          { name = "maturin"; package = maturin; }
          { name = "python"; package = pythonEnv; }
        ];

        packages = [
          toolchain
          pkgs.rust-analyzer-unwrapped
        ];

        env = [
          {
            name = "RUST_SRC_PATH";
            value = "${toolchain}/lib/rustlib/src/rust/library";
          }
        ];
      };

    # The development environment for testing the resulting python package.
    devShells.test = let pkgs = import nixpkgs {
      inherit system;
      overlays = [ devshell.overlays.default self.overlays.default ];
    }; in pkgs.devshell.mkShell {
      name = "relaxed-ik-test";

      commands = with pkgs; [
        {
          name = "python"; package = pkgs.python3.withPackages (ps: with ps; [
            relaxed-ik
          ]);
        }
      ];
    };
  });
}
