{ pkgs }:
let
  script = pkgs.writeShellScriptBin "acorn-hc"
  ''
  set -euxo pipefail
  holochain -c ./conductor-config.toml
  '';
in
{
 buildInputs = [ script ];
}
