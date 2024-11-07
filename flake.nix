{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { nixpkgs, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };
        rust-toolchain = pkgs.rust-bin.stable.latest.default;
        libraries = with pkgs; [
          cairo
          glib
          pango
          librsvg
          cargo-tauri
          atk.dev
          libappindicator
        ];

        buildInputs = with pkgs; [
          pkg-config
          rust-toolchain
          webkitgtk_4_1
          gtk3
          gdk-pixbuf
          dbus
          openssl
          curl
          wget
          dbus
          openssl_3
          glib
          gtk3
          gdk-pixbuf
          libsoup_3
          harfbuzz
          cairo
          webkitgtk
          librsvg
          nodePackages.pnpm
          nodejs_18
        ];
      in {
        devShells.default = pkgs.mkShell {
          inherit buildInputs;

          shellHook = ''
            export LD_LIBRARY_PATH=${
              pkgs.lib.makeLibraryPath libraries
            }:$LD_LIBRARY_PATH

            export XDG_DATA_DIRS=${pkgs.gsettings-desktop-schemas}/share/gsettings-schemas/${pkgs.gsettings-desktop-schemas.name}:${pkgs.gtk3}/share/gsettings-schemas/${pkgs.gtk3.name}:$XDG_DATA_DIRS
          '';
        };
      });
}
