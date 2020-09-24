use hdk3::prelude::*;
use dna_help::crud;

#[hdk_entry(id = "goal_vote")]
#[derive(Debug, Clone, PartialEq)]
pub struct GoalVote {
    goal_address: HeaderHash,
    urgency: f32,
    importance: f32,
    impact: f32,
    effort: f32,
    agent_address: AgentPubKey,
    unix_timestamp: u128,
}

crud!(GoalVote, goal_vote, "goal_vote");