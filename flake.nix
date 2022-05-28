{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
  };

  outputs = { self, nixpkgs, flake-utils, naersk }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = nixpkgs.legacyPackages."${system}";
        naersk-lib = naersk.lib."${system}";
      in
        rec {
          # `nix build / nix run`
          packages.arm-rs = naersk-lib.buildPackage {
            pname = "arm-rs";
            root = ./.;
            nativeBuildInputs = [
              pkgs.pkg-config
            ];
            buildInputs = [
              pkgs.systemd
            ];
          };
          defaultPackage = packages.arm-rs;

          # `nix develop`
          devShell = pkgs.mkShell {
            nativeBuildInputs = with pkgs; [
              pkg-config
              systemd

              rustc cargo cargo-edit ];
          };
        }
    );
}
