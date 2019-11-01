# acorn-hc
the holochain backend for Acorn, see [acorn-ui](https://github.com/h-be/acorn-ui) for user interface

Resources:
* [Acorn SoA Google Doc](https://docs.google.com/document/d/1VTne9BmrQgAgUV873pVm1yP2l--IMEGawfqnf5tpBaQ)
* [SoA Lightning Talk](https://www.youtube.com/embed/-z47R9wN5SQ?start=53&end=650&autoplay=1)

Before starting up the UI development, start up a Holochain Conductor with the Acorn DNA. Here's how:

Enter a nix shell (for convenience, we are using a `nix-hc` alias)

You can target the specific version of holonix that has the right version, by doing:
`nix-shell https://github.com/holochain/holonix/archive/v0.0.39.tar.gz`

Test that you are on Holochain version 0.0.34-alpha1
```
hc --version
holochain --version
```

Change directories to this project folder.
Run
```
hc package
```
This builds the DNA into the `dist` folder, from the source code under `zomes`.

Run
```
holochain -c ./conductor-config.toml
```
This starts up the Conductor with a running instance of the DNA in it.

Leave this terminal open and running, as long as you're doing development. Repackage and run `holochain ...` again if you make changes to the DNA.
