# Deprecated

## This code now lives at [https://github.com/h-be/acorn](https://github.com/h-be/acorn) in the `dna` and `conductor` folders.

# acorn-hc

[![CircleCI](https://circleci.com/gh/h-be/acorn-hc.svg?style=svg)](https://circleci.com/gh/h-be/acorn-hc)

Holochain backend for Acorn, see [acorn-ui](https://github.com/h-be/acorn-ui) for user interface, and [acorn-docs](https://github.com/h-be/acorn-docs) for general, design, and vision documentation.

To download it as a native application for Linux or MacOS from [acorn-release](https://github.com/h-be/acorn-release/releases).

## Holochain Compatibility

Current compatibility with [holochain/holochain](https://github.com/holochain/holochain):
[ad3ccd0a126683d5fa44985f7abc18f1d891de01 (Nov 9, 2020)](https://github.com/holochain/holochain/commit/ad3ccd0a126683d5fa44985f7abc18f1d891de01)

When changing, change this line in `install-holochain.sh`
```
REV=ad3ccd0a126683d5fa44985f7abc18f1d891de01
```

Also change this line in `Cargo.toml`:
```toml
hdk3 = { git = "https://github.com/holochain/holochain", rev = "ad3ccd0a126683d5fa44985f7abc18f1d891de01", package = "hdk3" }
```

Make sure the tests still pass for the new version.



## Running

Before starting up the UI development, start up a Holochain Conductor with the Acorn DNA. Here's how:

Enter a nix shell:

```
nix-shell --run acorn-package
nix-shell --run acorn-hc
```

This starts up the Conductor with a running instance of the DNA in it.

Leave this terminal open and running, as long as you're doing development.

## Building

To rebuild the DNA that holochain uses to run use the `hc` command:

```
nix-shell --run acorn-package
```

Stop the running conductor (ctrl + c) and rerun the above again if you make changes to the DNA.

## Testing

To run the tests

```
nix-shell --run acorn-test
```

## Formatting

To format the Rust code of both DNAs

```
nix-shell --run acorn-fmt
```

## Releasing

Edit the `version.current` of the `config.nix` file, and set it to the desired version number of the release.
Also edit the `version` property of `dnas/projects/zomes/acorn_projects/code/Cargo.toml` and `dnas/profiles/zomes/acorn_profiles/code/Cargo.toml`, and then run `nix-shell --run acorn-package` so that the `Cargo.lock` file is updated as well. 

> TODO: notes about CHANGELOG.md and CHANGELOG-UNRELEASED.md

> TODO: notes about updating Release notes

Run

```
nix-shell --run hn-release-github
```

## Updating

To update the holonix version (and therefore the holochain binaries) edit the holonix property of `config.nix`.
