pub mod edge;
pub mod entry_point;
pub mod goal;
pub mod goal_comment;
pub mod goal_member;
pub mod goal_vote;
pub mod member;
pub mod project_meta;

// send the direct messages that will result in
// signals being emitted to the UI
// fn notify_all(message: DirectMessage) -> ExternResult<()> {
//   fetch_members()?.iter().for_each(|member| {
//     // useful for development purposes
//     // uncomment to send signals to oneself
//     // if member.address == AGENT_ADDRESS.to_string() {
//     //     signal_ui(&message);
//     // }

//     if member.address != AGENT_ADDRESS.to_string() {
//       hdk::debug(format!(
//         "Send a message to: {:?}",
//         &member.address.to_string()
//       ))
//       .ok();
//       hdk::send(
//         Address::from(member.address.clone()),
//         JsonString::from(message.clone()).into(),
//         1.into(),
//       )
//       .ok();
//     }
//   });
//   Ok(())
// }

// fn notify_member(member: Member) -> ExternResult<()> {
//   let message = DirectMessage::NewMemberNotification(NewMemberSignalPayload {
//     member: member.clone(),
//   });
//   notify_all(message)
// }

// fn notify_entry_point(entry_point_response: EntryPointResponse) -> ExternResult<()> {
//   let message = DirectMessage::EntryPointNotification(EntryPointSignalPayload {
//     entry_point: entry_point_response.clone(),
//   });
//   notify_all(message)
// }

// fn notify_entry_point_archived(address: Address) -> ExternResult<()> {
//   let message =
//     DirectMessage::EntryPointArchivedNotification(EntryArchivedSignalPayload { address });
//   notify_all(message)
// }

// fn notify_goal_maybe_with_edge(goal_maybe_with_edge: GoalMaybeWithEdge) -> ExternResult<()> {
//   let message = DirectMessage::GoalMaybeWithEdgeNotification(GoalMaybeWithEdgeSignalPayload {
//     goal: goal_maybe_with_edge.clone(),
//   });
//   notify_all(message)
// }

// // handles create and update cases
// fn notify_edge(edge: GetResponse<Edge>) -> ExternResult<()> {
//   let message = DirectMessage::EdgeNotification(EdgeSignalPayload { edge });
//   notify_all(message)
// }

// fn notify_edge_archived(address: Address) -> ExternResult<()> {
//   let message = DirectMessage::EdgeArchivedNotification(EntryArchivedSignalPayload { address });
//   notify_all(message)
// }

// fn notify_goal_archived(archived: ArchiveGoalResponse) -> ExternResult<()> {
//   let message = DirectMessage::GoalArchivedNotification(GoalArchivedSignalPayload { archived });
//   notify_all(message)
// }

// fn notify_goal_comment(goal_comment: GetResponse<GoalComment>) -> ExternResult<()> {
//   let message = DirectMessage::GoalCommentNotification(GoalCommentSignalPayload { goal_comment });
//   notify_all(message)
// }

// fn notify_goal_member(goal_member: GetResponse<GoalMember>) -> ExternResult<()> {
//   let message = DirectMessage::GoalMemberNotification(GoalMemberSignalPayload { goal_member });
//   notify_all(message)
// }

// fn notify_goal_vote(goal_vote: GetResponse<GoalVote>) -> ExternResult<()> {
//   let message = DirectMessage::GoalVoteNotification(GoalVoteSignalPayload { goal_vote });
//   notify_all(message)
// }

// fn notify_goal_comment_archived(address: Address) -> ExternResult<()> {
//   let message =
//     DirectMessage::GoalCommentArchivedNotification(EntryArchivedSignalPayload { address });
//   notify_all(message)
// }

// fn notify_goal_member_archived(address: Address) -> ExternResult<()> {
//   let message =
//     DirectMessage::GoalMemberArchivedNotification(EntryArchivedSignalPayload { address });
//   notify_all(message)
// }

// fn notify_goal_vote_archived(address: Address) -> ExternResult<()> {
//   let message = DirectMessage::GoalVoteArchivedNotification(EntryArchivedSignalPayload { address });
//   notify_all(message)
// }
