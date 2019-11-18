{ pkgs }:
let
  script = pkgs.writeShellScriptBin "acorn-hc"
  ''
  set -euxo pipefail
  hc package
  holochain -c ./conductor-config.toml
  '';
in
{
 buildInputs = [ script ];
}
