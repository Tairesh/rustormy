{
  description = "Minimal neofetch-like weather CLI";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    naersk.url = "github:nix-community/naersk";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { nixpkgs, naersk, flake-utils, ... }:

    let
      mkPkg = system:
        let
          pkgs = import nixpkgs { inherit system; };
          naerskLib = pkgs.callPackage naersk { };
        in
        naerskLib.buildPackage {
          src = ./.;
          # 3. Убрали pkgs.glibc из buildInputs
          nativeBuildInputs = with pkgs; [ pkg-config ];
        };
    in
    (flake-utils.lib.eachDefaultSystem (system: {
      packages.default = mkPkg system;
      packages.rustormy = mkPkg system;
    }))
    // {

      homeManagerModules.rustormy = ./home-manager/home.nix;
    };
}
