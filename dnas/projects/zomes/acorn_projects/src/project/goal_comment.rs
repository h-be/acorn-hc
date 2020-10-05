use dna_help::{WrappedAgentPubKey, WrappedHeaderHash, crud};
use hdk3::prelude::*;

#[hdk_entry(id = "goal_comment")]
#[derive(Debug, Clone, PartialEq)]
pub struct GoalComment {
    goal_address: WrappedHeaderHash,
    content: String,
    agent_address: WrappedAgentPubKey,
    unix_timestamp: f64,
}

crud!(GoalComment, goal_comment, "goal_comment");
