{ pkgs }:
let
  acorn-hc = pkgs.writeShellScriptBin "acorn-hc"
  ''
  set -euxo pipefail
  holochain -c ./conductor-config.toml
  '';

  acorn-fmt = pkgs.writeShellScriptBin "acorn-fmt"
  ''
  set -euxo pipefail
  cd dnas/profiles/zomes/acorn_profiles/code
  cargo fmt
  cd ../../../../..
  cd dnas/projects/zomes/acorn_projects/code
  cargo fmt
  cd ../../../../..
  '';

  acorn-package = pkgs.writeShellScriptBin "acorn-package"
  ''
  set -euxo pipefail
  cd dnas/profiles
  hc package
  cd ../..
  cd dnas/projects
  hc package
  cd ../..
  '';
in
{
 buildInputs = [ acorn-hc acorn-fmt acorn-package];
}
