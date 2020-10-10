# This is an example of what downstream consumers of holonix should do
# This is also used to dogfood as many commands as possible for holonix
# For example the release process for holonix uses this file
let

 # point this to your local config.nix file for this project
 # example.config.nix shows and documents a lot of the options
 config = import ./config.nix;

 # START HOLONIX IMPORT BOILERPLATE
 holonix = import (
  if ! config.holonix.use-github
  then config.holonix.local.path
  else fetchTarball {
   url = "https://github.com/${config.holonix.github.owner}/${config.holonix.github.repo}/tarball/${config.holonix.github.ref}";
   sha256 = config.holonix.github.sha256;
  }
 ) { config = config; use-stable-rust = true; };
 # END HOLONIX IMPORT BOILERPLATE

in
with holonix.pkgs;
{
 dev-shell = stdenv.mkDerivation (holonix.shell // {
  name = "dev-shell";

  shellHook = holonix.pkgs.lib.concatStrings [
   holonix.shell.shellHook
   # NIX_ENFORCE_PURITY to fix = note: impure path `/[...]' used in link
   # https://nixos.wiki/wiki/Development_environment_with_nix-shell
    # cargo install --force holochain --git https://github.com/holochain/holochain.git --rev 4714fbddc48f1f9987c9f574b8d06510ea58ab16
    # cargo install --force dna_util --git https://github.com/holochain/holochain.git --rev 4714fbddc48f1f9987c9f574b8d06510ea58ab16
   ''
    export NIX_ENFORCE_PURITY=0
    cd test && npm install && cd ..
    ls test/node_modules/@holochain/tryorama
    npm install
   ''
  ];

  buildInputs = [ ]
   ++ holonix.shell.buildInputs
   ++ config.buildInputs
   ++ (holonix.pkgs.callPackage ./nix {
    holonix = holonix;
    config = config;
   }).buildInputs
  ;
 });
}
