use hdk3::prelude::*;
use dna_help::crud;

// A Goal Card. This is a card on the SoA Tree which can be small or non-small, complete or
// incomplete, certain or uncertain, and contains text content.
// user hash and unix timestamp are included to prevent hash collisions.
#[hdk_entry(id = "goal")]
#[derive(Debug, Clone, PartialEq)]
pub struct Goal {
  content: String,
  user_hash: AgentPubKey,
  user_edit_hash: Option<AgentPubKey>,
  timestamp_created: u128,
  timestamp_updated: Option<u128>,
  hierarchy: Hierarchy,
  status: Status,
  tags: Option<Vec<String>>,
  description: String,
  time_frame: Option<TimeFrame>,
}

#[derive(Serialize, Deserialize, Debug, SerializedBytes, Clone, PartialEq)]
pub enum Status {
  Uncertain,
  Incomplete,
  InProcess,
  Complete,
  InReview,
}

#[derive(Serialize, Deserialize, Debug, SerializedBytes, Clone, PartialEq)]
pub enum Hierarchy {
  Root,
  Trunk,
  Branch,
  Leaf,
  NoHierarchy,
}
#[derive(Serialize, Deserialize, Debug, SerializedBytes, Clone, PartialEq)]
pub struct TimeFrame {
  from_date: u128,
  to_date: u128,
}


crud!(Goal, goal, "goal");

#[derive(Serialize, Deserialize, Debug, SerializedBytes, Clone, PartialEq)]
pub struct ArchiveGoalResponse {
  address: HeaderHash,
  archived_edges: Vec<HeaderHash>,
  archived_goal_members: Vec<HeaderHash>,
  archived_goal_votes: Vec<HeaderHash>,
  archived_goal_comments: Vec<HeaderHash>,
  archived_entry_points: Vec<HeaderHash>,
}

#[derive(Serialize, Deserialize, Debug, SerializedBytes, Clone)]
pub struct GetHistoryResponse {
  entries: Vec<Goal>,
  members: Vec<Vec<super::goal_member::GoalMember>>,
  address: HeaderHash,
}





// DELETE
// pub fn archive_goal(address: HeaderHash) -> ExternResult<ArchiveGoalResponse> {
//   // commit the removeEntry. Returns the address of the removeEntry
//   delete_entry!(&address)?;

//   let archived_edges = inner_fetch_edges()?
//     .into_iter()
//     .filter(|get_response: &GetResponse<Edge>| {
//       // check whether the parent_address or child_address is equal to the given address.
//       // If so, the edge is connected to the goal being archived.
//       get_response.entry.child_address == address || get_response.entry.parent_address == address
//     })
//     .map(|get_response: GetResponse<Edge>| {
//       let edge_address = get_response.address;
//       // archive the edge with this address
//       match delete_entry!(&edge_address) {
//         Ok(_) => Ok(edge_address),
//         Err(e) => Err(e),
//       }
//     })
//     // filter out errors
//     .filter_map(Result::ok)
//     .collect(); // returns vec of the edge addresses which were removed

//   let archived_goal_members = archive_members_of_goal(&address)?; // returns vec of the goal_member addresses which were removed
//   let archived_goal_votes = inner_fetch_goal_votes()?
//     .into_iter()
//     .filter(|get_response: &GetResponse<GoalVote>| {
//       // check whether the parent_address or child_address is equal to the given address.
//       // If so, the edge is connected to the goal being archived.
//       get_response.entry.goal_address == address
//     })
//     .map(|get_response: GetResponse<GoalVote>| {
//       let goal_vote_address = get_response.address;
//       // archive the edge with this address
//       match delete_entry!(&goal_vote_address) {
//         Ok(_) => Ok(goal_vote_address),
//         Err(e) => Err(e),
//       }
//     })
//     // filter out errors
//     .filter_map(Result::ok)
//     .collect();
//   let archived_goal_comments = inner_fetch_goal_comments()?
//     .into_iter()
//     .filter(|get_response: &GetResponse<GoalComment>| {
//       // check whether the parent_address or child_address is equal to the given address.
//       // If so, the edge is connected to the goal being archived.
//       get_response.entry.goal_address == address
//     })
//     .map(|get_response: GetResponse<GoalComment>| {
//       let goal_comment_address = get_response.address;
//       // archive the edge with this address
//       match delete_entry!(&goal_comment_address) {
//         Ok(_) => Ok(goal_comment_address),
//         Err(e) => Err(e),
//       }
//     })
//     // filter out errors
//     .filter_map(Result::ok)
//     .collect(); // returns vec of the goal_member addresses which were removed
//                 // return the address of the archived goal for the UI to use
//   let archived_entry_points = inner_fetch_entry_points()?
//     .into_iter()
//     .filter(|entry_point_response: &EntryPointResponse| {
//       entry_point_response.entry_point.goal_address == address
//     })
//     .map(|entry_point_response: EntryPointResponse| {
//       let entry_point_address = entry_point_response.entry_point_address;
//       match delete_entry!(&entry_point_address) {
//         Ok(_) => Ok(entry_point_address),
//         Err(e) => Err(e),
//       }
//     })
//     // filter out errors
//     .filter_map(Result::ok)
//     .collect(); // returns vec of the entry_points addresses which were removed
//                 // return the address of the archived goal for the UI to use

//   let archive_response = ArchiveGoalResponse {
//     address,
//     archived_edges,
//     archived_goal_members,
//     archived_goal_votes,
//     archived_goal_comments,
//     archived_entry_points,
//   };
//   // notify_goal_archived(archive_response.clone())?;

//   Ok(archive_response)
// }




// pub fn history_of_goal(address: HeaderHash) -> ExternResult<GetHistoryResponse> {
//   let anchor_address = Entry::App(
//     "anchor".into(),       // app entry type
//     "goal_members".into(), // app entry value
//   )
//   .address();
//   // return all the Goal objects from the entries linked to the edge anchor (drop entries with wrong type)
//   let members = get_links!(
//     &anchor_address,
//     LinkMatch::Exactly("anchor->goal_member"), // the link type to match
//     LinkMatch::Any,
//   )?
//   // scoop all these entries up into an array and return it
//   .addresses()
//   .into_iter()
//   .map(|member_address: HeaderHash| {
//     if let Ok(Some(entry_history)) = hdk::api::get_entry_history(&member_address) {
//       Some(
//         entry_history
//           .items
//           .into_iter()
//           .map(|item| {
//             if let Some(App(_, value_entry)) = item.entry {
//               match serde_json::from_str::<GoalMember>(&Into::<String>::into(value_entry)).ok() {
//                 Some(goal_member) => {
//                   // filter down to only Goal Members that are associated with the requested Goal
//                   if goal_member.goal_address == address {
//                     Ok(goal_member)
//                   } else {
//                     Err(ZomeApiError::Internal("error".into()))
//                   }
//                 }
//                 None => Err(ZomeApiError::Internal("error".into())),
//               }
//             } else {
//               Err(ZomeApiError::Internal("error".into()))
//             }
//           })
//           .filter_map(Result::ok)
//           .collect(),
//       )
//     } else {
//       None
//     }
//   })
//   .filter_map(|op: Option<Vec<GoalMember>>| match op {
//     Some(vec) => {
//       if vec.len() > 0 {
//         Some(vec)
//       } else {
//         None
//       }
//     }
//     _ => None,
//   })
//   .collect();
//   if let Ok(Some(entry_history)) = hdk::api::get_entry_history(&address) {
//     Ok(GetHistoryResponse {
//       entries: entry_history
//         .items
//         .into_iter()
//         .map(|item| {
//           if let Some(App(_, value_entry)) = item.entry {
//             match serde_json::from_str::<Goal>(&Into::<String>::into(value_entry)).ok() {
//               Some(goal) => Ok(goal),
//               None => Err(ZomeApiError::Internal("error".into())),
//             }
//           } else {
//             Err(ZomeApiError::Internal("error".into()))
//           }
//         })
//         .filter_map(Result::ok)
//         .collect(),
//       members: members,
//       address: address,
//     })
//   } else {
//     Err(ZomeApiError::Internal("error".into()))
//   }
// }
