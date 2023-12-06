{
  inputs = {
    flake-parts.url = "github:hercules-ci/flake-parts";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    pre-commit-hooks-nix.url = "github:cachix/pre-commit-hooks.nix";
    pre-commit-hooks-nix.inputs.nixpkgs.follows = "nixpkgs";
    fenix.url = "github:nix-community/fenix";
    fenix.inputs.nixpkgs.follows = "nixpkgs";
    crane.url = "github:ipetkov/crane";
    crane.inputs.nixpkgs.follows = "nixpkgs";
    hercules-ci-effects.url = "github:hercules-ci/hercules-ci-effects";
    hercules-ci-effects.inputs.flake-parts.follows = "flake-parts";
  };

  outputs = inputs:
    inputs.flake-parts.lib.mkFlake {inherit inputs;} ({lib, ...}: {
      systems = ["x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin"];

      imports = [
        inputs.pre-commit-hooks-nix.flakeModule
        inputs.hercules-ci-effects.flakeModule
      ];

      perSystem = {
        config,
        pkgs,
        lib,
        inputs',
        system,
        ...
      }: let
        rust = inputs'.fenix.packages.default;

        darwinDeps = lib.attrValues {
          inherit (pkgs) libiconv;
          inherit (pkgs.darwin.apple_sdk.frameworks) SystemConfiguration;
        };

        crane = inputs.crane.lib.${system}.overrideToolchain rust.toolchain;
        craneArgs = {
          pname = "aoc-cli";
          version = "unstable-2023-12-05";
          src = crane.cleanCargoSource (crane.path ./.);
          strictDeps = true;
          buildInputs = lib.optionals pkgs.stdenv.isDarwin darwinDeps;
        };
        cargoArtifacts = crane.buildDepsOnly (craneArgs
          // {
            cargoExtraArgs = "--workspace";
          });
      in {
        packages = rec {
          aoc-cli = crane.buildPackage (craneArgs
            // {
              inherit cargoArtifacts;
              meta.mainProgram = "aoc";
            });
          default = aoc-cli;
          rust-toolchain = rust.toolchain;
        };

        pre-commit.settings.tools =
          lib.mapAttrs
          (_: lib.mkForce)
          {
            inherit (pkgs) alejandra;
          };
        pre-commit.settings.hooks = {
          alejandra.enable = true;
          deadnix.enable = true;
          nil.enable = true;
          # TODO: This is broken on CI
          # rustfmt.enable = true;
          # clippy.enable = true;
          # cargo-check.enable = true;
        };

        devShells.default = pkgs.mkShell {
          packages = [rust.toolchain] ++ lib.optionals pkgs.stdenv.isDarwin darwinDeps;
          shellHook = ''
            ${config.pre-commit.installationScript}
          '';
        };

        checks = {
          clippy = crane.cargoClippy (craneArgs
            // {
              inherit cargoArtifacts;
              cargoExtraArgs = "--workspace";
              cargoClippyExtraArgs = "--all-targets -- --deny warnings";
            });

          rustfmt = crane.cargoFmt (craneArgs
            // {
              cargoExtraArgs = "--all";
            });
        };

        formatter = pkgs.alejandra;
      };

      herculesCI.ciSystems = lib.mkForce ["x86_64-linux" "aarch64-linux"];
    });
}
