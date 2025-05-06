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
        p.rust-bin.nightly.latest.default.override {
          targets = ["wasm32-unknown-unknown"];
        });

      simple-3d-wasm = craneLib.buildPackage {
        src = craneLib.cleanCargoSource ./simple-3d-wasm;
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
        # inherit simple-3d-wasm;
      };

      # packages.default = simple-3d-wasm;
      #
      # apps.default = flake-utils.lib.mkApp {
      #   drv = pkgs.writeShellScriptBin "simple-3d-wasm" ''
      #     ${pkgs.wasmtime}/bin/wasmtime run ${simple-3d-wasm}/bin/custom-toolchain.wasm
      #   '';
      # };

      devShells.default = let
        alias-build-wasm = pkgs.writeShellScriptBin "bw" ''${pkgs.cargo-watch}/bin/cargo-watch -C simple-3d-wasm -s "wasm-pack build --target web" -c'';
        alias-build-cli = pkgs.writeShellScriptBin "bc" ''${pkgs.cargo-watch}/bin/cargo-watch -C simple-3d-cli -x run -c'';
        alias-test = pkgs.writeShellScriptBin "t" ''${pkgs.cargo-watch}/bin/cargo-watch -x test -c'';
        alias-profile = pkgs.writeShellScriptBin "p" ''cargo bench'';
        alias-serve = pkgs.writeShellScriptBin "s" ''${pkgs.python3}/bin/python3 -m http.server 9999 -d ./simple-3d-wasm'';
      in
        craneLib.devShell {
          checks = self.checks.${system};
          buildInputs = [alias-build-wasm alias-build-cli alias-test alias-profile alias-serve];
          packages = [];
        };
    });
}
