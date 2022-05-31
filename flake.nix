{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    #mozillapkgs.url = "github:mozilla/nixpkgs-mozilla";
    fenix.url = "github:nix-community/fenix";
  };

  outputs = { self, nixpkgs, flake-utils, naersk, fenix, ... }@inputs:
    flake-utils.lib.eachSystem [ "x86_64-linux" ] (system: let
    #flake-utils.lib.eachDefaultSystem (system: let
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

      packWithTests = doCheck: naersk-lib.buildPackage {
        pname = "arm-rs";
        root = ./.;

        inherit doCheck;
        doDoc = true;
        doDocFail = true;

        nativeBuildInputs = [ pkgs.pkg-config ];
        buildInputs = [ pkgs.systemd ];
        cargoTestCommands = x:
          x ++ [
            # clippy
            ''cargo clippy --all --all-features --tests -- \
              -D clippy::pedantic \
              -D warnings \
              -A clippy::module-name-repetitions \
              -A clippy::too-many-lines \
              -A clippy::cast-possible-wrap \
              -A clippy::cast-possible-truncation \
              -A clippy::nonminimal_bool''
          ];
      };
    in rec {
      # `nix build`
      packages = rec {
        arm-rs = packWithTests false;
        default = arm-rs;
        doc = arm-rs.doc;
      };

      # `nix run`
      apps = rec {
        arm-rs = flake-utils.lib.mkApp { drv = packages.arm-rs; };
        default = arm-rs;
      };

      # `nix develop`
      devShells.default = pkgs.mkShell {
        inputsFrom = [
          packages.arm-rs
        ];

        nativeBuildInputs = [
          rust
          pkgs.cargo-edit
        ];
      };

      # `nix flake check`
      checks = {
        buildPack = packWithTests true;
      };
    }
  );
}
