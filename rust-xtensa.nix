{
  autoPatchelfHook,
  stdenv,
  fetchurl,
  libgcc,
  libz,
  patchelf,
  arch ? "x86_64-unknown-linux-gnu",
}: let
  version = "1.88.0.0";
  base_url = "https://github.com/esp-rs/rust-build/releases/download";

  rustSrc = fetchurl {
    url = "${base_url}/v${version}/rust-src-${version}.tar.xz";
    sha256 = "sha256-m35u//UHO7uFtQ5mn/mVhNuJ1PCsuljgkD3Rmv3uuaE=";
  };
in
  stdenv.mkDerivation {
    pname = "rust-xtensa";
    inherit version;

    src = fetchurl {
      url = "${base_url}/v${version}/rust-${version}-${arch}.tar.xz";
      sha256 = "sha256-dFNJFHSl9yiyRIFlHUPLzq+S9438q+fLiCxr8h/uBQU=";
    };

    nativeBuildInputs = [
      autoPatchelfHook
      patchelf
    ];

    buildInputs = [
      libgcc
      libz
      stdenv.cc.cc.lib
    ];

    installPhase = ''
      runHook preInstall
      mkdir -p $out
      cp -r cargo/* $out/
      cp -r rustc/* $out/
      cp -r clippy-preview/* $out/
      cp -r rust-docs/* $out/
      cp -r rust-docs-json-preview/* $out/
      cp -r rustfmt-preview/* $out/
      cp -r rust-std-${arch}/* $out/

      # Install rust-src so -Zbuild-std works
      tar -xJf ${rustSrc} -C $out --strip-components=2

      runHook postInstall
    '';
  }
