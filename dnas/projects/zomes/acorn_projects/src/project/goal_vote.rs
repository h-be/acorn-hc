use dna_help::{WrappedAgentPubKey, WrappedHeaderHash, crud};
use hdk3::prelude::*;

#[hdk_entry(id = "goal_vote")]
#[derive(Debug, Clone, PartialEq)]
pub struct GoalVote {
    goal_address: WrappedHeaderHash,
    urgency: f64,
    importance: f64,
    impact: f64,
    effort: f64,
    agent_address: WrappedAgentPubKey,
    unix_timestamp: f64,
}

crud!(GoalVote, goal_vote, "goal_vote");
