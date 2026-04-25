{
  description = "ESP32-S3 no_std Rust dev shell";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = {
    self,
    nixpkgs,
  }: let
    system = "x86_64-linux";
    pkgs = import nixpkgs {inherit system;};
    rust-xtensa = pkgs.callPackage ./rust-xtensa.nix {};
  in {
    devShells.${system}.default = pkgs.mkShell {
      packages = [
        rust-xtensa
        (pkgs.callPackage ./xtensa-gcc.nix {})
        pkgs.cargo
        pkgs.rustc
        pkgs.rust-analyzer
        pkgs.espflash
      ];
    };
  };
}
