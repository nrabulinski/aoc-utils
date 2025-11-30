{
  inputs = {
    flake-parts.url = "github:hercules-ci/flake-parts";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    pre-commit-hooks-nix.url = "github:cachix/pre-commit-hooks.nix";
    pre-commit-hooks-nix.inputs.nixpkgs.follows = "nixpkgs";
    fenix.url = "github:nix-community/fenix";
    fenix.inputs.nixpkgs.follows = "nixpkgs";
    crane.url = "github:ipetkov/crane";
  };

  outputs = inputs:
    inputs.flake-parts.lib.mkFlake {inherit inputs;} {
      systems = ["x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin"];

      imports = [
        inputs.pre-commit-hooks-nix.flakeModule
      ];

      perSystem = {
        config,
        pkgs,
        lib,
        inputs',
        ...
      }: let
        rust = inputs'.fenix.packages.default;

        crane = (inputs.crane.mkLib pkgs).overrideToolchain rust.toolchain;
        craneArgs = {
          pname = "aoc-cli";
          version = "unstable-2025-11-30";
          src = crane.cleanCargoSource (crane.path ./.);
          strictDeps = true;
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
          cargo-check = {
            enable = true;
            package = rust.toolchain;
          };
        };

        devShells.default = pkgs.mkShell {
          packages = [rust.toolchain pkgs.cargo-edit];
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
    };
}
