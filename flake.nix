{
  description = "Virtual audio devices and an echo-cancelling bridge for macOS";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-26.05";

  outputs = { self, nixpkgs }:
    let
      systems = [ "aarch64-darwin" "x86_64-darwin" ];
      forAll = f: nixpkgs.lib.genAttrs systems (system: f nixpkgs.legacyPackages.${system});
    in
    {
      packages = forAll (pkgs: {
        default = pkgs.rustPlatform.buildRustPackage {
          pname = "audioctl";
          version = (builtins.fromTOML (builtins.readFile ./Cargo.toml)).workspace.package.version;
          src = self;
          cargoLock.lockFile = ./Cargo.lock;
          cargoBuildFlags = [ "--workspace" ];
          CODESIGN = "/usr/bin/codesign";
          DRIVER_ARCHS = if pkgs.stdenv.hostPlatform.isAarch64 then "arm64" else "x86_64";
          doCheck = false;
          meta = {
            description = "Virtual audio devices and an echo-cancelling bridge for macOS";
            license = pkgs.lib.licenses.gpl3Plus;
            platforms = systems;
            mainProgram = "audioctl";
          };
        };
      });
    };
}
