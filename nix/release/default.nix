{ pkgs, config }:
let

 tag = "v${config.release.version.current}";

 script = pkgs.writeShellScriptBin "release-github-wasm"
 ''
 set -euxo pipefail
 export artifact='acorn.dna.json'
 export tag=''${CIRCLE_TAG:-${tag}}
 hc package -o "$TMP/$artifact"
 github-release upload --file "$TMP/$artifact" --owner ${config.release.github.owner} --repo ${config.release.github.repo} --tag $tag --name $artifact --token $GITHUB_DEPLOY_TOKEN
 '';
in
{
 buildInputs = [ script ];
}
