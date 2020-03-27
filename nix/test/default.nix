{ pkgs }:
let
  script = pkgs.writeShellScriptBin "acorn-test"
  ''
  set -euxo pipefail
  acorn-package
  hc test --skip-package
  '';
in
{
 buildInputs = [ script ];
}
