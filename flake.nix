{
  description = "Rust flake";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable"; # or whatever vers
  };

  outputs =
    { nixpkgs, ... }:
    let
      for_all_systems = nixpkgs.lib.genAttrs nixpkgs.lib.systems.doubles.all;
    in
    {
      devShells = for_all_systems (system: {
        default = nixpkgs.legacyPackages.${system}.mkShell {
          packages = with nixpkgs.legacyPackages.${system}; [ cargo ];
        };
      });
    };
}
