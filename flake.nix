{
  description = "A Rust library for generating temporary email addresses.";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    systems.url = "github:nix-systems/default";
    flake-utils.url = "github:numtide/flake-utils";
    flake-utils.inputs.systems.follows = "systems";
    dream2nix.url = "github:nix-community/dream2nix";
    dream2nix.inputs.nixpkgs.follows = "nixpkgs";
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    dream2nix,
    rust-overlay,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [(import rust-overlay)];
      };
      lib = pkgs.lib;
    in {
      packages = {
        rquest = dream2nix.lib.evalModules {
          packageSets.nixpkgs = pkgs;
          modules = [
            ./default.nix
            {
              rust-crane.buildCommand = lib.mkForce "build --features use-rquest --no-default-features --examples";
            }
          ];
        };
        default = dream2nix.lib.evalModules {
          packageSets.nixpkgs = pkgs;
          modules = [
            ./default.nix
          ];
        };
      };
      devShells.default = pkgs.mkShell {
        packages = [
          pkgs.bacon
          pkgs.cargo-readme
        ];
        inputsFrom = [
          self.packages.${system}.default.devShell
        ];
      };
    });
}
