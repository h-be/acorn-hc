{ pkgs }:
let
  script = pkgs.writeShellScriptBin "acorn-test"
  ''
  set -euxo pipefail
  hc test
  '';
in
{
 buildInputs = [ script ];
}
