{ pkgs }:
let
  acorn-hc = pkgs.writeShellScriptBin "acorn-hc"
  ''
  set -euxo pipefail
  # don't error if it doesn't exist to remove
  rm --force my-conductor-config.toml
  cp conductor-config.toml my-conductor-config.toml
  hc hash --path dnas/profiles/dist/profiles.dna.json | awk '/DNA Hash: /{print $NF}' | tr -d '\n' > profiles_dna_address
  node update-dna-address.js
  holochain -c ./my-conductor-config.toml
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
