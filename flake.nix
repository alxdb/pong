{
  inputs = {
    utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    nixpkgs.url = "nixpkgs/nixos-unstable";
  };
  outputs =
    {
      self,
      fenix,
      utils,
      nixpkgs,
    }:
    utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        toolchain = fenix.packages.${system}.default;
      in
      {
        devShell = pkgs.mkShell rec {
          nativeBuildInputs = [
            (toolchain.withComponents [
              "cargo"
              "clippy"
              "rustc"
              "rustfmt"
            ])
            pkgs.wayland
            pkgs.libxkbcommon
            pkgs.alsa-lib
            pkgs.pkg-config
            pkgs.systemd
            pkgs.vulkan-headers
            pkgs.vulkan-loader
            pkgs.vulkan-validation-layers
          ];
          LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath nativeBuildInputs}";
        };
      }
    );
}
