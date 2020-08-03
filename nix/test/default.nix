{ pkgs }:
let
  script = pkgs.writeShellScriptBin "acorn-test"
  ''
  set -euxo pipefail
  acorn-package
  HC_IGNORE_SIM2H_URL_PROPERTY=true hc test --skip-package
  '';
in
{
 buildInputs = [ script ];
}
