{
  stdenv,
  fetchurl,
  autoPatchelfHook,
  patchelf,
  zlib,
  ncurses5,
  libxml2,
  libgcc,
  stdenv_cc ? stdenv.cc.cc.lib,
}: let
  version = "14.2.0_20260121";
  url = "https://github.com/espressif/crosstool-NG/releases/download/esp-${version}/xtensa-esp-elf-${version}-x86_64-linux-gnu.tar.gz";
in
  stdenv.mkDerivation {
    pname = "xtensa-esp32s3-elf-gcc";
    inherit version;

    src = fetchurl {
      inherit url;
      sha256 = "sha256-TgkKbL8f93aWhNmiSMm4vb5MCtoJit2lS0xnqRRJr6c=";
    };

    nativeBuildInputs = [
      autoPatchelfHook
      patchelf
    ];

    buildInputs = [
      stdenv_cc
      libgcc
      zlib
      ncurses5
      libxml2
    ];

    dontConfigure = true;
    dontBuild = true;

    installPhase = ''
      mkdir -p $out
      tar -xzf $src -C $out --strip-components=1
    '';
  }
