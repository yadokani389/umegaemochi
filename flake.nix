{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    treefmt-nix.url = "github:numtide/treefmt-nix";
    flake-parts.url = "github:hercules-ci/flake-parts";
    systems.url = "github:nix-systems/default";
    git-hooks-nix.url = "github:cachix/git-hooks.nix";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    inputs@{
      nixpkgs,
      flake-parts,
      ...
    }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = with inputs; [
        treefmt-nix.flakeModule
        git-hooks-nix.flakeModule
      ];
      systems = import inputs.systems;

      perSystem =
        {
          config,
          pkgs,
          system,
          ...
        }:
        let
          rust-toolchain = pkgs.rust-bin.stable.latest.default.override {
            targets = [
              "x86_64-unknown-linux-gnu"
              "x86_64-linux-android"
              "aarch64-linux-android"
              "armv7-linux-androideabi"
              "i686-linux-android"
            ];
          };

          androidComposition = pkgs.androidenv.composeAndroidPackages {
            cmdLineToolsVersion = "13.0";
            toolsVersion = "26.1.1";
            platformToolsVersion = "35.0.1";
            buildToolsVersions = [
              "30.0.3"
              "34.0.0"
            ];
            includeEmulator = false;
            platformVersions = [
              "28"
              "29"
              "30"
              "33"
              "34"
            ];
            includeSources = false;
            includeSystemImages = false;
            systemImageTypes = [ "google_apis_playstore" ];
            abiVersions = [
              "armeabi-v7a"
              "arm64-v8a"
            ];
            cmakeVersions = [ "3.10.2" ];
            includeNDK = true;
            ndkVersions = [ "26.3.11579264" ];
            useGoogleAPIs = false;
            useGoogleTVAddOns = false;
            includeExtras = [ "extras;google;gcm" ];
          };

        in
        rec {
          _module.args.pkgs = import nixpkgs {
            inherit system;
            overlays = [ inputs.rust-overlay.overlays.default ];
            config = {
              allowUnfree = true;
              android_sdk.accept_license = true;
            };
          };

          packages.umegaemochi = pkgs.callPackage ./package.nix { };

          packages.default = packages.umegaemochi;

          devShells.default = pkgs.mkShell {
            inputsFrom = [ config.pre-commit.devShell ];
            inherit (packages.default) buildInputs;
            inherit (packages.default) nativeBuildInputs;
            packages = [ rust-toolchain ];
          };

          devShells.android = pkgs.mkShell {
            inputsFrom = [ config.pre-commit.devShell ];
            inherit (packages.default) buildInputs;
            inherit (packages.default) nativeBuildInputs;
            packages = [
              rust-toolchain
              androidComposition.androidsdk
              pkgs.jdk17
              pkgs.gradle
            ];

            ANDROID_HOME = "${androidComposition.androidsdk}/libexec/android-sdk";
            NDK_HOME = "${androidComposition.androidsdk}/libexec/android-sdk/ndk-bundle";
            GRADLE_OPTS = "-Dorg.gradle.project.android.aapt2FromMavenOverride=${androidComposition.androidsdk}/libexec/android-sdk/build-tools/34.0.0/aapt2";
            XDG_DATA_DIRS = "${pkgs.gsettings-desktop-schemas}/share/gsettings-schemas/${pkgs.gsettings-desktop-schemas.name}:${pkgs.gtk3}/share/gsettings-schemas/${pkgs.gtk3.name}";
          };

          treefmt = {
            projectRootFile = "flake.nix";
            programs = {
              nixfmt.enable = true;
              rustfmt.enable = true;
              rustfmt.edition = "2021";
              taplo.enable = true;
            };

            settings.formatter = {
              taplo.options = [
                "fmt"
                "-o"
                "reorder_keys=true"
              ];
            };
          };

          pre-commit = {
            check.enable = true;
            settings = {
              hooks = {
                ripsecrets.enable = true;
                typos.enable = true;
                treefmt.enable = true;
                clippy-tauri = {
                  enable = true;
                  name = "clippy-tauri";
                  entry = "${pkgs.rust-bin.stable.latest.default}/bin/cargo-clippy --offline --all --all-targets --manifest-path src-tauri/Cargo.toml -- -Dwarnings";
                  files = "\\.(rs)$";
                  pass_filenames = false;
                };
              };
            };
          };
        };
    };
}
