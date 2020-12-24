#!/bin/bash

REV=3f1c535a5f2aec621c1550088cb4d5169677d812

cargo install --force holochain \
  --git https://github.com/holochain/holochain.git \
  --rev $REV
cargo install --force dna_util \
  --git https://github.com/holochain/holochain.git \
  --rev $REV