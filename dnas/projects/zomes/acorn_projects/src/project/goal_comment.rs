use hdk3::prelude::*;
use dna_help::crud;

#[hdk_entry(id = "goal_comment")]
#[derive(Debug, Clone, PartialEq)]
pub struct GoalComment {
  goal_address: HeaderHash,
  content: String,
  agent_address: AgentPubKey,
  unix_timestamp: u128,
}

crud!(GoalComment, goal_comment, "goal_comment");