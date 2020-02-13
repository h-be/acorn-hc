# acorn-hc

[![CircleCI](https://circleci.com/gh/h-be/acorn-hc.svg?style=svg)](https://circleci.com/gh/h-be/acorn-hc)

the holochain backend for Acorn, see [acorn-ui](https://github.com/h-be/acorn-ui) for user interface

Resources:

- [Acorn SoA Google Doc](https://docs.google.com/document/d/1VTne9BmrQgAgUV873pVm1yP2l--IMEGawfqnf5tpBaQ)
- [SoA Lightning Talk](https://www.youtube.com/embed/-z47R9wN5SQ?start=53&end=650&autoplay=1)

## Running

Before starting up the UI development, start up a Holochain Conductor with the Acorn DNA. Here's how:

Enter a nix shell:

```
nix-shell --run acorn-hc
```

This starts up the Conductor with a running instance of the DNA in it.

Leave this terminal open and running, as long as you're doing development.

## Building

To rebuild the DNA that holochain uses to run use the `hc` command:

```
nix-shell --run 'hc package'
```

Stop the running conductor (ctrl + c) and rerun the above again if you make changes to the DNA.

## Testing

To run the tests

```
nix-shell --run acorn-test
```

## Releasing

Edit the `version.current` of the `config.nix` file, and set it to the desired version number of the release.

> TODO: notes about CHANGELOG.md and CHANGELOG-UNRELEASED.nd

Run

```
nix-shell --run hn-release-github
```

## Updating

To update the holonix version (and therefore the holochain binaries) edit the holonix property of `config.nix`.
