{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    #mozillapkgs.url = "github:mozilla/nixpkgs-mozilla";
    fenix.url = "github:nix-community/fenix";
  };

  outputs = { self, nixpkgs, flake-utils, naersk, fenix, ... }@inputs:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [ fenix.overlay ];
      };
      rust = pkgs.fenix.fromToolchainFile {
        file = ./rust-toolchain.toml;
        sha256 = "sha256-qAAsuHw8IXejRJ5EdRXUavrSWkIYrp2s+Ozv9Zo/8zo=";
      };
      naersk-lib = naersk.lib."${system}".override {
        cargo = rust;
        rustc = rust;
      };
    in rec {
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
        inputsFrom = [
          packages.arm-rs
        ];

        nativeBuildInputs = [
          rust
          pkgs.cargo-edit
        ];
      };
    }
  );
}
