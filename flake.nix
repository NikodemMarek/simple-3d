{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    crane.url = "github:ipetkov/crane";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    crane,
    flake-utils,
    rust-overlay,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [(import rust-overlay)];
      };

      craneLib = (crane.mkLib pkgs).overrideToolchain (p:
        p.rust-bin.stable.latest.default.override {
          targets = ["wasm32-unknown-unknown"];
        });

      simple-3d = craneLib.buildPackage {
        src = craneLib.cleanCargoSource ./.;
        strictDeps = true;
        cargoExtraArgs = "--target wasm32-unknown-unknown";

        doCheck = false;

        buildInputs =
          [
          ]
          ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
            pkgs.libiconv
          ];
      };
    in {
      checks = {
        inherit simple-3d;
      };

      packages.default = simple-3d;

      apps.default = flake-utils.lib.mkApp {
        drv = pkgs.writeShellScriptBin "simple-3d" ''
          ${pkgs.wasmtime}/bin/wasmtime run ${simple-3d}/bin/custom-toolchain.wasm
        '';
      };

      devShells.default = let
        alias-build = pkgs.writeShellScriptBin "b" ''${pkgs.cargo-watch}/bin/cargo-watch -s "wasm-pack build --target web" -c'';
        alias-test = pkgs.writeShellScriptBin "t" ''${pkgs.cargo-watch}/bin/cargo-watch -x test -c'';
        alias-serve = pkgs.writeShellScriptBin "s" ''${pkgs.python3}/bin/python3 -m http.server 9999'';
      in
        craneLib.devShell {
          checks = self.checks.${system};
          buildInputs = [alias-build alias-test alias-serve];
          packages = [];
        };
    });
}
