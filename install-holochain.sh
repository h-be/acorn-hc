#!/bin/bash

REV=65e05a5b95596d056f3b40dc5ce0e9fec2c5f073

cargo install --force holochain \
  --git https://github.com/holochain/holochain.git \
  --rev $REV
cargo install --force dna_util \
  --git https://github.com/holochain/holochain.git \
  --rev $REV