use dna_help::{
  crud,
  WrappedHeaderHash
};
use hdk3::prelude::*;

// An edge. This is an arrow on the SoA Tree which directionally links
// two goals.
#[hdk_entry(id = "edge")]
#[derive(Debug, Clone, PartialEq)]
pub struct Edge {
    parent_address: WrappedHeaderHash,
    child_address: WrappedHeaderHash,
    randomizer: f64,
}

crud!(Edge, edge, "edge");
