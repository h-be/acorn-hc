#!/bin/bash

REV=15dd4ba75ae02d02b48a05da012e8fbeabe2f48e

cargo install --force holochain \
  --git https://github.com/holochain/holochain.git \
  --rev $REV
cargo install --force dna_util \
  --git https://github.com/holochain/holochain.git \
  --rev $REV