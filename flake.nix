{
  description = "Minimal neofetch-like weather CLI";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    naersk.url = "github:nix-community/naersk";
    home-manager.url = "github:nix-community/home-manager";
  };

  outputs =
    {
      nixpkgs,
      naersk,
      ...
    }:
    let
      pkgs = nixpkgs.legacyPackages."x86_64-linux";
      naerskLib = pkgs.callPackage naersk { };

      rustormyPkg = naerskLib.buildPackage {
        src = ./.;
        buildInputs = [ pkgs.glibc ];
        nativeBuildInputs = [ pkgs.pkg-config ];
      };

      rustormyModule = import ./home-manager/home.nix;
    in
    {
      packages.x86_64-linux.default = rustormyPkg;

      homeManagerModules.rustormy = rustormyModule;
    };
}
