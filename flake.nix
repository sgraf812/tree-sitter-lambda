{
  description = "Tree-sitter parser for taschenrechner";

  inputs.nixpkgs.url = "github:nixos/nixpkgs";
  inputs.flake-utils.url = "github:numtide/flake-utils";

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
      in {
        devShell = pkgs.mkShell rec {
          name = "tree-sitter-taschenrechner";
          buildInputs = with pkgs; [
            nodejs
            tree-sitter
            cargo
            rustc
          ];
        };
      }
    );
}
