{
  description = "The wayland first terminal emulator.";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    systems.url = "github:nix-systems/default-linux";
    naersk.url = "github:nix-community/naersk";
  };

  outputs = { nixpkgs, systems, naersk, ... }:
    let eachSystem = nixpkgs.lib.genAttrs (import systems);
    in {
      packages = eachSystem (system:
        let pkgs = nixpkgs.legacyPackages.${system};
        in rec {
          wurm = pkgs.callPackage ./derivation.nix {
            naersk = pkgs.callPackage naersk { };
          };
          default = wurm;
        });
      devShell = eachSystem (system:
        let pkgs = nixpkgs.legacyPackages.${system};
        in pkgs.mkShell rec {
          buildInputs = with pkgs; [
            pkg-config
            libxkbcommon
            libGL
            wayland
          ];
          LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath buildInputs}";
        });
    };
}
