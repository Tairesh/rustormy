{
  pkgs,
  lib,
  config,
  ...
}:

let
  inherit (lib)
    mkEnableOption
    mkPackageOption
    mkOption
    literalExpression
    ;
  cfg = config.programs.rustormy;
  tomlFormat = pkgs.formats.toml { };
  defaultSettings = {
    providers = [ "open_meteo" ];
    live_mode = false;
    live_mode_interval = 300;
    use_geocoding_cache = false;
    verbose = 0;
    connect_timeout = 10;

    api_keys = {
      open_weather_map = "";
      world_weather_online = "";
      weather_api = "";
      weather_bit = "";
      tomorrow_io = "";
      open_uv = "";
    };

    format = {
      output_format = "text";
      text_mode = "full";
      use_colors = false;
      show_city_name = false;
      align_right = false;
      wind_in_degrees = false;
      units = "metric";
      language = "en";

      color_theme = {
        label = "BrightBlue";
        location = "BrightWhite";
        temperature = "BrightYellow";
        wind = "BrightRed";
        precipitation = "BrightCyan";
        pressure = "BrightGreen";
        humidity = "Blue";
      };
    };
  };
in
{
  options.programs.rustormy = {
    enable = mkEnableOption "Minimal neofetch-like weather CLI";

    package = mkPackageOption pkgs "rustormy" { nullable = true; };

    settings = mkOption {
      type = tomlFormat.type;
      default = { };
      example = literalExpression ''
        {
          city = "London";
          lat = 51.5074
          live-mode = false
          api-keys = {
            open_weather_map = "";
          };
        }
      '';
    };
  };
  config = lib.mkIf cfg.enable {
    home.packages = lib.mkIf (cfg.package != null) [ cfg.package ];

    xdg.configFile."rustormy/config.toml" = {
      source = tomlFormat.generate "config.toml" (
        lib.recursiveUpdate defaultSettings (cfg.settings or { })
      );
    };
  };

}
