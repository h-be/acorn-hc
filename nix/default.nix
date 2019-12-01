{ holonix }:
{
 buildInputs = []
 ++ ( holonix.pkgs.callPackage ./holochain { pkgs = holonix.pkgs; }).buildInputs
 ++ ( holonix.pkgs.callPackage ./test { pkgs = holonix.pkgs; }).buildInputs;
}
