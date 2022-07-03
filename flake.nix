{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    fenix.url = "github:nix-community/fenix";
  };

  outputs = { self, nixpkgs, flake-utils, naersk, fenix, ... }@inputs:
  flake-utils.lib.eachSystem [ "x86_64-linux" ] (system: let
    pname = "arm-rs";
    overlay = _: prev: {
      ${pname} = prev.callPackage ./nix/default.nix { inherit naersk; };
    };
    pkgs = import nixpkgs {
      inherit system;
      overlays = [ fenix.overlay overlay ];
      config.allowUnfree = true;
    };
  in rec {
      # `nix build`
      packages = {
        ${pname} = pkgs.${pname};
        default = packages.${pname};
        doc = packages.${pname}.doc;
      };

      # `nix run`
      apps = {
        ${pname} = flake-utils.lib.mkApp { drv = packages.${pname}; };
        default = apps.${pname};
      };

      nixosModules.default = import ./nix/module.nix;
      overlays.default = overlay;

      # `nix develop`
      devShells.default = pkgs.mkShell {
        inputsFrom = [ pkgs.${pname} ];

        nativeBuildInputs = with pkgs; [
          cargo-edit
          handbrake
          makemkv
        ];
      };

      # `nix flake check`
      checks.default = pkgs.${pname};
    }
  );
}
