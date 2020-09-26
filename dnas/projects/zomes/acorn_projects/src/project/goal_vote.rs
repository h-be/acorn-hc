use hdk3::prelude::*;
use dna_help::crud;

#[hdk_entry(id = "goal_vote")]
#[derive(Debug, Clone, PartialEq)]
pub struct GoalVote {
    goal_address: HeaderHash,
    urgency: f64,
    importance: f64,
    impact: f64,
    effort: f64,
    agent_address: AgentPubKey,
    unix_timestamp: f64,
}

crud!(GoalVote, goal_vote, "goal_vote");