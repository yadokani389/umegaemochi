{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    old-nixpkgs.url =
      "github:NixOS/nixpkgs/0c19708cf035f50d28eb4b2b8e7a79d4dc52f6bb";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { nixpkgs, old-nixpkgs, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [
            (final: prev: {
              inherit (old-nixpkgs.legacyPackages.${system}) webkitgtk_4_1;
            })
            rust-overlay.overlays.default
          ];
          config = {
            allowUnfree = true;
            android_sdk.accept_license = true;
          };
        };

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
          buildToolsVersions = [ "30.0.3" "34.0.0" ];
          includeEmulator = false;
          platformVersions = [ "28" "29" "30" "33" "34" ];
          includeSources = false;
          includeSystemImages = false;
          systemImageTypes = [ "google_apis_playstore" ];
          abiVersions = [ "armeabi-v7a" "arm64-v8a" ];
          cmakeVersions = [ "3.10.2" ];
          includeNDK = true;
          ndkVersions = [ "26.3.11579264" ];
          useGoogleAPIs = false;
          useGoogleTVAddOns = false;
          includeExtras = [ "extras;google;gcm" ];
        };

      in rec {
        packages.default = pkgs.callPackage ./package.nix { };
        devShells.default = pkgs.mkShell {
          inherit (packages.default) buildInputs;
          nativeBuildInputs = packages.default.nativeBuildInputs
            ++ (with pkgs; [ rustc gcc rustfmt clippy ]);
        };

        devShells.android = pkgs.mkShell rec {
          inherit (packages.default) buildInputs;
          nativeBuildInputs = packages.default.nativeBuildInputs
            ++ [ rust-toolchain androidComposition.androidsdk ]
            ++ (with pkgs; [ jdk17 gradle ]);

          ANDROID_HOME = "${androidComposition.androidsdk}/libexec/android-sdk";
          NDK_HOME = "${ANDROID_HOME}/ndk-bundle";
          GRADLE_OPTS =
            "-Dorg.gradle.project.android.aapt2FromMavenOverride=${androidComposition.androidsdk}/libexec/android-sdk/build-tools/34.0.0/aapt2";
          XDG_DATA_DIRS =
            "${pkgs.gsettings-desktop-schemas}/share/gsettings-schemas/${pkgs.gsettings-desktop-schemas.name}:${pkgs.gtk3}/share/gsettings-schemas/${pkgs.gtk3.name}";
        };
      });
}
