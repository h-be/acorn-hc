#!/bin/bash

REV=2dcb4d77c3909b3d94b46ffca385f7cc41c2f00c

cargo install --force holochain \
  --git https://github.com/holochain/holochain.git \
  --rev $REV
cargo install --force dna_util \
  --git https://github.com/holochain/holochain.git \
  --rev $REV