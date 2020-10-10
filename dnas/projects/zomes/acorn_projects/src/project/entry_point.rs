use dna_help::{WrappedAgentPubKey, WrappedHeaderHash, crud};
use hdk3::prelude::*;

// The "Entry" in EntryPoint is not a reference to Holochain "Entries"
// it is rather the concept of an Entrance, as in a doorway, to the tree
#[hdk_entry(id = "entry_point")]
#[derive(Debug, Clone, PartialEq)]
pub struct EntryPoint {
    color: String,
    creator_address: WrappedAgentPubKey,
    created_at: f64,
    goal_address: WrappedHeaderHash,
}

crud!(EntryPoint, entry_point, "entry_point");