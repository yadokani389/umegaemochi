{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    old-nixpkgs.url =
      "github:NixOS/nixpkgs/0c19708cf035f50d28eb4b2b8e7a79d4dc52f6bb";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { nixpkgs, old-nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [
            (final: prev: {
              inherit (old-nixpkgs.legacyPackages.${system}) webkitgtk_4_1;
            })
          ];
        };
      in { defaultPackage = pkgs.callPackage ./package.nix { }; });
}
