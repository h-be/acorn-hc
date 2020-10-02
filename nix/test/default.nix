{ pkgs }:
let
  script = pkgs.writeShellScriptBin "acorn-test"
  ''
  set -euxo pipefail
  acorn-package
  cd test
  npm run test
  '';
in
{
 buildInputs = [ script ];
}
