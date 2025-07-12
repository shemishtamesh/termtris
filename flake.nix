{
  description = "Rust flake";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable"; # or whatever vers
  };

  outputs =
    { nixpkgs, ... }:
    let
      pkgs = system: nixpkgs.legacyPackages.${system};
      for_all_systems = nixpkgs.lib.genAttrs nixpkgs.lib.systems.doubles.all;
    in
    {
      devShells = for_all_systems (system: {
        default = (pkgs system).mkShell {
          packages = with (pkgs system); [ cargo ];
        };
      });
      packages = for_all_systems (system: {
        default = (pkgs system).rustPlatform.buildRustPackage {
          name = "termtris";
          src = ./.;
          cargoHash = "sha256-ZeKC0QGwn+wCel/mtSxldjVSWGbuoduJ5Crq56zXpw0=";
        };
      });
    };
}
