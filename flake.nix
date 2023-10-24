{
  description = "Is prime number rust backend";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs = inputs@{ self, nixpkgs, rust-overlay, flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems =
        [ "x86_64-linux" "aarch64-linux" "aarch64-darwin" "x86_64-darwin" ];
      perSystem = { config, self', inputs', pkgs, system, ... }:
        let
          crane = rec {
            lib = self.inputs.crane.lib.${system};
            stable = lib.overrideToolchain self'.packages.rust-stable;
          };
        in {
          packages = rec {
            rust-stable = inputs'.rust-overlay.packages.rust.override {
              extensions = [ "rust-src" "rust-analyzer" "clippy" ];
            };
            default = bin;
            bin = crane.stable.buildPackage {
              src = ./is-prime-number;
              cargoBuildCommand = "cargo build --release";
              buildInputs = pkgs.lib.optionals pkgs.stdenv.isDarwin (with pkgs.darwin.apple_sdk.frameworks; [ Security ]);
            };
            image = pkgs.dockerTools.buildLayeredImage {
              name = "is-prime-number";
              config.Cmd = [ "${bin}/bin/is-prime-number"];
            };
          };
          devShells = {
            default = pkgs.mkShell {
              buildInputs = [ self'.packages.rust-stable ]
                ++ (with pkgs; [ bacon rnix-lsp hyperfine cargo-flamegraph docker-client])
                ++ (pkgs.lib.optionals pkgs.stdenv.isDarwin (with pkgs.darwin.apple_sdk.frameworks; [ Security ]));
            };
          };
        };
    };
}
