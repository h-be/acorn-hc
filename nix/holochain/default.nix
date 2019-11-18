{ pkgs }:
let
  script-cluster = pkgs.writeShellScriptBin "acorn-hc"
  ''
  set -euxo pipefail
  hc package
  holochain -c ./conductor-config.toml
  '';
in
{
 buildInputs = [ script-cluster ];
}
