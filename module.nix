{
  config,
  pkgs,
  lib ? pkgs.lib,
  ...
}:
with lib; let
  cfg = config.services.alanp-web;
  basePath = "/var/lib/alanp-web";
in {
  options = {
    services.alanp-web = {
      enable = mkOption {
        type = types.bool;
        default = false;
        description = ''
          Whether to run alanp.me.
        '';
      };
      package = mkOption {
        type = types.path;
        defaultText = literalExpression "pkgs.alanp-web";
        description = "The website package.";
      };
      listenHost = mkOption {
        type = types.str;
        default = "0.0.0.0";
        example = "localhost";
        description = ''
          The hostname or address to listen on.
        '';
      };
      port = mkOption {
        type = types.int;
        default = 3000;
        description = ''
          TCP port the web server should listen to.
        '';
      };
      projectGit = mkOption {
        type = types.str;
        default = "https://github.com/alanpq/website.git";
        example = "https://github.com/alanpq/website.git";
        description = ''The git url to pull the projects/awards from. (must have subfolder 'projects'/'awards' for sparse checkout)'';
      };
    };
  };

  config = mkIf cfg.enable {
    systemd.tmpfiles.rules = [
      "d ${basePath} 0750 alanp-web alanp-web"
    ];

    users.extraGroups.alanp-web = {};
    users.extraUsers.alanp-web = {
      description = "alanp.me website";
      group = "alanp-web";
      home = basePath;
      isSystemUser = true;
      useDefaultShell = true;
    };

    systemd.services.alanp-web = {
      wantedBy = ["multi-user.target"];
      environment = {
        PATH_ROOT = "${basePath}/www";
        PORT = toString cfg.port;
        HOST = cfg.listenHost;
      };
      path = [pkgs.gitMinimal];
      preStart = ''
        mkdir -m 0700 -p ${basePath}/www
        cd ${basePath}/www
        git init
        git remote get-url origin || git remote add -f origin ${cfg.projectGit}
        git config core.sparseCheckout true
        echo "projects/" > .git/info/sparse-checkout
        echo "awards/" >> .git/info/sparse-checkout
        echo "static/" >> .git/info/sparse-checkout
        echo "src/templates" >> .git/info/sparse-checkout
        echo "src/styles" >> .git/info/sparse-checkout
        git reset --hard
        git pull --force origin main
      '';
      serviceConfig = {
        ExecStart = "${cfg.package}/bin/website";
        Type = "simple";
        Restart = "always";
        User = "alanp-web";
      };
    };
  };
}
