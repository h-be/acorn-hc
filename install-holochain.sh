#!/bin/bash

REV=6bd822cf3378178b5600ab79d8560f04b5a5b837

cargo install --force holochain \
  --git https://github.com/holochain/holochain.git \
  --rev $REV
cargo install --force dna_util \
  --git https://github.com/holochain/holochain.git \
  --rev $REV