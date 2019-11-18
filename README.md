# acorn-hc
the holochain backend for Acorn, see [acorn-ui](https://github.com/h-be/acorn-ui) for user interface

Resources:
* [Acorn SoA Google Doc](https://docs.google.com/document/d/1VTne9BmrQgAgUV873pVm1yP2l--IMEGawfqnf5tpBaQ)
* [SoA Lightning Talk](https://www.youtube.com/embed/-z47R9wN5SQ?start=53&end=650&autoplay=1)

Before starting up the UI development, start up a Holochain Conductor with the Acorn DNA. Here's how:

Enter a nix shell:
```
nix-shell --run acorn-hc
```
This packages the DNA and starts up the Conductor with a running instance of the DNA in it.

Leave this terminal open and running, as long as you're doing development.

Stop (ctrl + c) and rerun the above again if you make changes to the DNA.

To update the holonix version (and therefore the holochain binaries) edit the holonix property of `config.nix`.
