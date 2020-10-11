use dna_help::{WrappedAgentPubKey, WrappedHeaderHash, crud};
use hdk3::prelude::*;

#[hdk_entry(id = "goal_comment")]
#[derive(Debug, Clone, PartialEq)]
pub struct GoalComment {
    pub goal_address: WrappedHeaderHash,
    pub content: String,
    pub agent_address: WrappedAgentPubKey,
    pub unix_timestamp: f64,
}

crud!(GoalComment, goal_comment, "goal_comment");
