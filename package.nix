{ lib, stdenv, cargo-tauri, libsoup_3, nodejs, openssl, pkg-config, pnpm
, rustPlatform, webkitgtk_4_1, wrapGAppsHook4, }:

stdenv.mkDerivation (finalAttrs: {
  pname = "umegaemochi";
  version = "0.1.0";

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
    hash = "sha256-AtY97KFXctIVJ0a15CBgefWPYagvIGGWfC6jWkjU+zk=";
  };

  cargoRoot = "src-tauri";
  buildAndTestSubdir = finalAttrs.cargoRoot;

  cargoDeps = rustPlatform.fetchCargoTarball {
    inherit (finalAttrs) pname version src;
    sourceRoot = "${finalAttrs.src.name}/${finalAttrs.cargoRoot}";
    hash = "sha256-KuNx/fhkLNIrP3+AXJRT6d27ygZWYrSr7OrDXaQm0SA=";
  };

  nativeBuildInputs = [
    cargo-tauri.hook
    nodejs
    pkg-config
    pnpm.configHook
    rustPlatform.cargoSetupHook
    wrapGAppsHook4
  ];

  buildInputs = [ libsoup_3 openssl webkitgtk_4_1 ];

  meta = {
    license = lib.licenses.mit;
    mainProgram = "umegaemochi";
    platforms = lib.platforms.linux;
  };
})
