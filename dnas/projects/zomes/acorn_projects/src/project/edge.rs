use dna_help::crud;
use hdk3::prelude::*;

// An edge. This is an arrow on the SoA Tree which directionally links
// two goals.
#[hdk_entry(id = "edge")]
#[derive(Debug, Clone, PartialEq)]
pub struct Edge {
    parent_address: HeaderHash,
    child_address: HeaderHash,
    randomizer: f64,
}

crud!(Edge, edge, "edge");
