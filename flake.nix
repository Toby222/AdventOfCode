{
  description = "A Nix-flake-based Rust development environment";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";

    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      treefmt-nix,
      fenix,
    }:
    let
      supportedSystems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];
      forEachSupportedSystem =
        f:
        nixpkgs.lib.genAttrs supportedSystems (
          system:
          f (
            let
              pkgs = import nixpkgs { inherit system; };
              fenix' = (import fenix { inherit system pkgs; });
              toolchain = fenix'.minimal;
            in
            {
              inherit pkgs toolchain;
            }
          )
        );
    in
    {
      formatter = forEachSupportedSystem (
        { pkgs, ... }: (treefmt-nix.lib.evalModule pkgs ./treefmt.nix).config.build.wrapper
      );
      devShells = forEachSupportedSystem (
        { pkgs, toolchain }:
        {
          default = pkgs.mkShell {
            LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
              pkgs.openssl
            ];
            PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
            packages =
              [
                pkgs.pkg-config
              ]
              ++ [
                toolchain.cargo
                toolchain.rustc
              ];
          };
        }
      );
    };
}
