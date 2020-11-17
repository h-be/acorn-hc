#!/bin/bash

REV=ad3ccd0a126683d5fa44985f7abc18f1d891de01

cargo install --force holochain \
  --git https://github.com/holochain/holochain.git \
  --rev $REV
cargo install --force dna_util \
  --git https://github.com/holochain/holochain.git \
  --rev $REV