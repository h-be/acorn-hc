use hdk3::prelude::*;
use dna_help::crud;

// a relationship between a Goal and an Agent
#[hdk_entry(id = "goal_member")]
#[derive(Debug, Clone, PartialEq)]
pub struct GoalMember {
  goal_address: HeaderHash,
  agent_address: AgentPubKey,
  user_edit_hash: Option<AgentPubKey>,
  unix_timestamp: u128,
}

crud!(GoalMember, goal_member, "goal_member");

// DELETE
// clear all members
// pub fn archive_goal_members(address: &HeaderHash) -> ExternResult<Vec<HeaderHash>> {
//   fetch_goal_members()?
//     .into_iter()
//     .filter(|wire_entry: WireEntry| {
//       // check whether the parent_address or child_address is equal to the given address.
//       // If so, the edge is connected to the goal being archived.
//       wire_entry.entry.goal_address == *address
//     })
//     .map(|wire_entry: WireEntry| {
//       let goal_member_address = wire_entry.address;
//       // archive the edge with this address
//       match delete_entry!(&goal_member_address) {
//         Ok(_) => {
//           // notify_goal_member_archived(goal_member_address.clone())?;
//           Ok(goal_member_address)
//         }
//         Err(e) => Err(e),
//       }
//     })
//     // filter out errors
//     .collect()
// }