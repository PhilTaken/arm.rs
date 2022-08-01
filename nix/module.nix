{ config
, lib
, pkgs
, ...
}:
with lib;

let
  cfg = config.services.arm-rs;
in {
  options.services.arm-rs = {
    enable = mkEnableOption "arm-rs";

    config = mkOption {
      type = types.submodule;
      default = {};
    };

    package = mkOption {
      type = types.package;
      default = pkgs.arm-rs;
    };

    # config for handbrake license, etc
  };

  config = mkIf cfg.enable {
    boot.kernelModules = [ "sg" ];

    systemd.services.arm-rs = {
      description = "Arm-rs service";
      after = [ "network-online.target" ];
      wantedBy = [ "multi-user.target" ];

      serviceConfig = {
        User = "arm";
        Group = "arm";
        Restart = "always";

        ExecStart = "${cfg.package}/bin/arm-rs";
      };
    };

    users.extraUsers.arm = {
      isSystemUser = true;
      group = "arm";
      home = "/home/arm";
      createHome = true;
    };

    users.extraGroups.arm = {};
  };
}
