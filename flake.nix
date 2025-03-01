{
  description = "Build a cargo project";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    systems.url = "github:nix-systems/default";
    dream2nix.url = "github:nix-community/dream2nix";
    dream2nix.inputs.nixpkgs.follows = "nixpkgs";
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = {
    self,
    nixpkgs,
    systems,
    dream2nix,
    rust-overlay,
    ...
  }: let
    forEachSystem = function: (nixpkgs.lib.genAttrs (import systems) (system:
      function rec {
        inherit system;
        pkgs = import nixpkgs {
          inherit system;
          overlays = [(import rust-overlay)];
        };
        inherit (pkgs) lib stdenv;
      }));
  in {
    packages = forEachSystem ({pkgs, ...}: rec {
      rustToolchain = pkgs.rust-bin.stable.latest.default.override {
        extensions = ["rust-analyzer" "rust-src"];
      };
      default = dream2nix.lib.evalModules {
        packageSets.nixpkgs = pkgs;
        modules = [
          ({config, ...}: {
            imports = [
              dream2nix.modules.dream2nix.rust-cargo-lock
              dream2nix.modules.dream2nix.rust-cargo-vendor
              dream2nix.modules.dream2nix.rust-crane
            ];

            mkDerivation = {
              src = ./.;
              nativeBuildInputs = [
                config.deps.openssl.dev
                config.deps.pkg-config
              ];
            };
            rust-crane = {
              buildCommand = "build --examples";
              depsDrv = {
                mkDerivation.nativeBuildInputs = [
                  config.deps.openssl.dev
                  config.deps.pkg-config
                  config.deps.git
                  config.deps.cmake
                  config.deps.clang
                ];
                env.LIBCLANG_PATH = "${config.deps.libclang.lib}/lib";
              };
            };

            env.LIBCLANG_PATH = "${config.deps.libclang.lib}/lib";

            deps = {nixpkgs, ...}: {
              cargo = rustToolchain;
              inherit
                (nixpkgs)
                stdenv
                openssl
                pkg-config
                git
                cmake
                clang
                libclang
                ;
            };
            inherit ((builtins.fromTOML (builtins.readFile (self + /Cargo.toml))).package) name version;
          })
        ];
      };
    });
    devShells = forEachSystem ({
      pkgs,
      system,
      ...
    }: {
      default = pkgs.mkShell {
        packages = [self.packages.${system}.rustToolchain];
        inputsFrom = [self.packages.${system}.default.devShell];
        RUST_SRC_PATH = "${self.packages.${system}.rustToolchain}/lib/rustlib/src/rust/library";
        LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";
      };
    });
  };
}
