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
    pub parent_address: WrappedHeaderHash,
    pub child_address: WrappedHeaderHash,
    pub randomizer: f64,
}

crud!(Edge, edge, "edge");
