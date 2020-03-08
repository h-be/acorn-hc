{ pkgs }:
let
  script = pkgs.writeShellScriptBin "acorn-test"
  ''
  set -euxo pipefail
  rm -rf dist
  mkdir dist
  hc package -o dist/acorn.dna.json
  hc test --skip-package
  '';
in
{
 buildInputs = [ script ];
}
