{ pkgs, config }:
let

 tag = "v${config.release.version.current}";

 script = pkgs.writeShellScriptBin "release-github-wasm"
 ''
 set -euxo pipefail
 export profiles_artifact='./dnas/profiles/dist/profiles.dna.json'
 export projects_artifact='./dnas/projects/dist/projects.dna.json'
 export tag=''${CIRCLE_TAG:-${tag}}
 acorn-package
 github-release upload --file "$profiles_artifact" --owner ${config.release.github.owner} --repo ${config.release.github.repo} --tag $tag --name $artifact --token $GITHUB_DEPLOY_TOKEN
 github-release upload --file "$projects_artifact" --owner ${config.release.github.owner} --repo ${config.release.github.repo} --tag $tag --name $artifact --token $GITHUB_DEPLOY_TOKEN
 '';
in
{
 buildInputs = [ script ];
}
