{ pkgs }:
let
  acorn-hc = pkgs.writeShellScriptBin "acorn-hc"
  ''
  set -euxo pipefail
  acorn-package
  npm run start
  '';

  acorn-fmt = pkgs.writeShellScriptBin "acorn-fmt"
  ''
  set -euxo pipefail
  cargo fmt
  '';

  acorn-package = pkgs.writeShellScriptBin "acorn-package"
  ''
  set -euxo pipefail
  cargo build --release --target wasm32-unknown-unknown
  cd dnas/profiles && dna-util -c profiles.dna.workdir
  cd ../..
  cd dnas/projects && dna-util -c projects.dna.workdir
  '';
in
{
 buildInputs = [ acorn-hc acorn-fmt acorn-package];
}
