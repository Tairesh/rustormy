{
  description = "Minimal neofetch-like weather CLI";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    naersk.url = "github:nix-community/naersk";
    home-manager.url = "github:nix-community/home-manager";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { nixpkgs, naersk, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };

        naerskLib = pkgs.callPackage naersk { };

        rustormyPkg = naerskLib.buildPackage {
          src = ./.;
          buildInputs = [ pkgs.glibc ];
          nativeBuildInputs = [ pkgs.pkg-config ];
        };

        rustormyModule = import ./home-manager/home.nix;
      in
      {
        packages.default = rustormyPkg;
        packages.rustormy = rustormyPkg;

        homeManagerModules.rustormy = rustormyModule;
      }
    );
}

