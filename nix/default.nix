{ holonix }:
{
 buildInputs = []
 ++( holonix.pkgs.callPackage ./holochain { pkgs = holonix.pkgs; }).buildInputs;
}
