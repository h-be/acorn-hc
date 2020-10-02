use dna_help::crud;
use hdk3::prelude::*;

#[hdk_entry(id = "goal_comment")]
#[derive(Debug, Clone, PartialEq)]
pub struct GoalComment {
    goal_address: HeaderHash,
    content: String,
    agent_address: AgentPubKey,
    unix_timestamp: f64,
}

crud!(GoalComment, goal_comment, "goal_comment");
