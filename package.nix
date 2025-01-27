{ lib, stdenv, cargo-tauri, glib-networking, nodejs, openssl, pkg-config, pnpm
, rustPlatform, webkitgtk_4_1, wrapGAppsHook4, }:

stdenv.mkDerivation (finalAttrs: {
  pname = "umegaemochi";
  version = "0.4.0";

  src = lib.fileset.toSource {
    root = ./.;
    fileset = lib.fileset.difference ./. (lib.fileset.unions [
      (lib.fileset.maybeMissing ./result)
      ./package.nix
      ./flake.nix
      ./flake.lock
    ]);
  };

  pnpmDeps = pnpm.fetchDeps {
    inherit (finalAttrs) src pname version;
    hash = "sha256-oaJazr7bUEcvG2X8vQy9ewMIpvKyJ6C612hc1lyqFsE=";
  };

  cargoRoot = "src-tauri";
  buildAndTestSubdir = finalAttrs.cargoRoot;

  cargoDeps = rustPlatform.fetchCargoTarball {
    inherit (finalAttrs) pname version src;
    sourceRoot = "${finalAttrs.src.name}/${finalAttrs.cargoRoot}";
    hash = "sha256-RX162oOuDeOY0aLBkDff5KdxM5FmK2LE+c18tjQzvko=";
  };

  nativeBuildInputs = [
    cargo-tauri.hook
    nodejs
    pkg-config
    pnpm.configHook
    rustPlatform.cargoSetupHook
    wrapGAppsHook4
  ];

  buildInputs = [ glib-networking openssl webkitgtk_4_1 ];

  meta = {
    license = lib.licenses.mit;
    mainProgram = "umegaemochi";
    platforms = lib.platforms.linux;
  };
})
