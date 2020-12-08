#!/bin/bash

cargo build --release --target wasm32-unknown-unknown
cd dnas/profiles && dna-util -c profiles.dna.workdir
cd ../..
cd dnas/projects && dna-util -c projects.dna.workdir