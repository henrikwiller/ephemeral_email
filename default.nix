{
  config,
  dream2nix,
  ...
}: {
  imports = [
    dream2nix.modules.dream2nix.rust-cargo-lock
    dream2nix.modules.dream2nix.rust-cargo-vendor
    dream2nix.modules.dream2nix.rust-crane
  ];
  deps = {nixpkgs, ...}: {
    mkRustToolchain = pkgs: (pkgs.rust-bin.stable.latest.default.override {
      extensions = ["rust-analyzer" "rust-src"];
    });
    inherit
      (nixpkgs)
      pkg-config
      openssl
      git
      cmake
      clang
      libclang
      ;
  };

  inherit ((builtins.fromTOML (builtins.readFile ./Cargo.toml)).package) name version;

  mkDerivation = {
    src = ./.;
    nativeBuildInputs = [
      config.deps.pkg-config
      config.deps.openssl.dev
    ];
    shellHook = ''
      export LIBCLANG_PATH="${config.deps.libclang.lib}/lib"
      export RUST_SRC_PATH="${config.deps.crane.cargo}/lib/rustlib/src/rust/library"
    '';
  };
  env.LIBCLANG_PATH = "${config.deps.libclang.lib}/lib";

  rust-crane = {
    buildCommand = "build --examples";
    depsDrv = {
      mkDerivation = {
        nativeBuildInputs = [
          config.deps.pkg-config
          config.deps.openssl.dev
          config.deps.git
          config.deps.cmake
          config.deps.clang
        ];
        shellHook = ''
          export LIBCLANG_PATH="${config.deps.libclang.lib}/lib"
        '';
      };
      env.LIBCLANG_PATH = "${config.deps.libclang.lib}/lib";
    };
  };
}
