use dna_help::{WrappedAgentPubKey, WrappedHeaderHash, crud};
use hdk3::prelude::*;

// a relationship between a Goal and an Agent
#[hdk_entry(id = "goal_member")]
#[derive(Debug, Clone, PartialEq)]
pub struct GoalMember {
  pub goal_address: WrappedHeaderHash,
  pub agent_address: WrappedAgentPubKey,
  pub user_edit_hash: Option<WrappedAgentPubKey>,
  pub unix_timestamp: f64,
}

crud!(GoalMember, goal_member, "goal_member");

// DELETE
// clear all members
pub fn archive_goal_members(address: WrappedHeaderHash) -> ExternResult<Vec<WrappedHeaderHash>> {
  Ok(inner_fetch_goal_members()?
    .0.into_iter()
    .filter(|wire_entry: &GoalMemberWireEntry| {
      // check whether the parent_address or child_address is equal to the given address.
      // If so, the edge is connected to the goal being archived.
      wire_entry.entry.goal_address == address.clone()
    })
    .map(|wire_entry: GoalMemberWireEntry| {
      let goal_member_address = wire_entry.address;
      // archive the edge with this address
      match inner_archive_goal_member(goal_member_address.clone()) {
        Ok(_) => {
          // notify_goal_member_archived(goal_member_address.clone())?;
          Ok(goal_member_address)
        }
        Err(e) => Err(e),
      }
    })
    // filter out errors
    .filter_map(Result::ok)
    .collect())
}
