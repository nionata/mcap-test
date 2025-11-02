{
  description = "Rust development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        # This sets up nixpkgs, where we will pull our dependencies from
        pkgs = (import nixpkgs) {
          # You can insert overlays here by calling `inherit system overlays;` 
          inherit system;
        };
      in
      {
        # This will be entered by direnv, or by manually running `nix shell`. This ensures
        # that our development environment will have all the correct tools at the correct
        # version for this project.
        devShell = pkgs.mkShell {
          # Here we add any tools that we want in our dev-shell but aren't required to build
          # our application.
          nativeBuildInputs = with pkgs;
            [
              nixpkgs-fmt
              rustc
              cargo
              clippy
            ];
        };
      }
    );
}