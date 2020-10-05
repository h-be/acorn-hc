use dna_help::crud;
use hdk3::prelude::*;

// The "Entry" in EntryPoint is not a reference to Holochain "Entries"
// it is rather the concept of an Entrance, as in a doorway, to the tree
#[hdk_entry(id = "entry_point")]
#[derive(Debug, Clone, PartialEq)]
pub struct EntryPoint {
    color: String,
    creator_address: AgentPubKey,
    created_at: f64,
    goal_address: EntryHash,
}

crud!(EntryPoint, entry_point, "entry_point");