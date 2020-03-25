#![feature(proc_macro_hygiene)]
#[macro_use]
extern crate hdk;
extern crate hdk_proc_macros;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate holochain_json_derive;

use hdk::{
    entry_definition::ValidatingEntryType,
    error::ZomeApiResult,
    holochain_json_api::{error::JsonError, json::JsonString},
    holochain_persistence_api::cas::content::Address,
    // AGENT_ADDRESS, AGENT_ID_STR,
    // AGENT_ADDRESS,
};
use std::convert::TryInto;

use hdk_proc_macros::zome;

mod anchor;
mod project;

use project::{
    GetResponse, ProjectMeta, Member, ArchiveGoalResponse, Edge, GetHistoryResponse, Goal, GoalComment, GoalMaybeWithEdge,
    GoalMember, GoalVote, EntryPointResponse, EntryPoint
};
//The GetResponse struct allows our zome functions to return an entry along with its
//address so that Redux can know the address of goals and edges

// these types will come straight through signals to the UI,
// so they will actually be referenced there. Be mindful of this
pub const NEW_MEMBER_SIGNAL_TYPE: &str = "new_member";
pub const ENTRY_POINT_SIGNAL_TYPE: &str = "entry_point";
pub const ENTRY_POINT_ARCHIVED_SIGNAL_TYPE: &str = "entry_point_archived";
pub const GOAL_MAYBE_WITH_EDGE_SIGNAL_TYPE: &str = "goal_maybe_with_edge";
pub const GOAL_ARCHIVED_SIGNAL_TYPE: &str = "goal_archived";
pub const GOAL_COMMENT_SIGNAL_TYPE: &str = "goal_comment";
pub const GOAL_COMMENT_ARCHIVED_SIGNAL_TYPE: &str = "goal_comment_archived";
pub const GOAL_MEMBER_SIGNAL_TYPE: &str = "goal_member";
pub const GOAL_MEMBER_ARCHIVED_SIGNAL_TYPE: &str = "goal_member_archived";
pub const GOAL_VOTE_SIGNAL_TYPE: &str = "goal_vote";
pub const GOAL_VOTE_ARCHIVED_SIGNAL_TYPE: &str = "goal_vote_archived";

#[derive(Clone, Debug, Serialize, Deserialize, DefaultJson, PartialEq)]
#[serde(rename_all = "camelCase")]
struct NewMemberSignalPayload {
    member: Member,
}

#[derive(Clone, Debug, Serialize, Deserialize, DefaultJson, PartialEq)]
#[serde(rename_all = "camelCase")]
struct EntryPointSignalPayload {
    entry_point: EntryPointResponse,
}

#[derive(Clone, Debug, Serialize, Deserialize, DefaultJson, PartialEq)]
#[serde(rename_all = "camelCase")]
struct GoalMaybeWithEdgeSignalPayload {
    goal: GoalMaybeWithEdge,
}

#[derive(Clone, Debug, Serialize, Deserialize, DefaultJson, PartialEq)]
#[serde(rename_all = "camelCase")]
struct GoalArchivedSignalPayload {
    archived: ArchiveGoalResponse,
}

#[derive(Clone, Debug, Serialize, Deserialize, DefaultJson, PartialEq)]
#[serde(rename_all = "camelCase")]
struct GoalCommentSignalPayload {
    goal_comment: GetResponse<GoalComment>,
}

#[derive(Clone, Debug, Serialize, Deserialize, DefaultJson, PartialEq)]
#[serde(rename_all = "camelCase")]
struct GoalMemberSignalPayload {
    goal_member: GetResponse<GoalMember>,
}

#[derive(Clone, Debug, Serialize, Deserialize, DefaultJson, PartialEq)]
#[serde(rename_all = "camelCase")]
struct GoalVoteSignalPayload {
    goal_vote: GetResponse<GoalVote>,
}

// Used for GoalComment, GoalMember, GoalVote, and EntryPoint
#[derive(Clone, Debug, Serialize, Deserialize, DefaultJson, PartialEq)]
#[serde(rename_all = "camelCase")]
struct EntryArchivedSignalPayload {
    address: Address,
}

/// Fully typed definition of the types of direct messages.
/// All of which exist for the purposes of UI signals
/// at this time.
#[derive(Clone, Serialize, Deserialize, Debug, DefaultJson, PartialEq)]
pub(crate) enum DirectMessage {
    NewMemberNotification(NewMemberSignalPayload),
    EntryPointNotification(EntryPointSignalPayload),
    GoalMaybeWithEdgeNotification(GoalMaybeWithEdgeSignalPayload),
    GoalArchivedNotification(GoalArchivedSignalPayload),
    GoalCommentNotification(GoalCommentSignalPayload),
    GoalMemberNotification(GoalMemberSignalPayload),
    GoalVoteNotification(GoalVoteSignalPayload),
    EntryPointArchivedNotification(EntryArchivedSignalPayload),
    GoalCommentArchivedNotification(EntryArchivedSignalPayload),
    GoalMemberArchivedNotification(EntryArchivedSignalPayload),
    GoalVoteArchivedNotification(EntryArchivedSignalPayload),
}

// send a signal to the UI
pub(crate) fn signal_ui(message: &DirectMessage) {
    match message {
        // Members
        DirectMessage::NewMemberNotification(signal_payload) => {
            hdk::emit_signal(NEW_MEMBER_SIGNAL_TYPE, signal_payload).ok();
        }
        // EntryPoints
        DirectMessage::EntryPointNotification(signal_payload) => {
            hdk::emit_signal(ENTRY_POINT_SIGNAL_TYPE, signal_payload).ok();
        }
        DirectMessage::EntryPointArchivedNotification(signal_payload) => {
            hdk::emit_signal(ENTRY_POINT_ARCHIVED_SIGNAL_TYPE, signal_payload).ok();
        }
        // Goals
        DirectMessage::GoalMaybeWithEdgeNotification(signal_payload) => {
            hdk::emit_signal(GOAL_MAYBE_WITH_EDGE_SIGNAL_TYPE, signal_payload).ok();
        }
        DirectMessage::GoalArchivedNotification(signal_payload) => {
            hdk::emit_signal(GOAL_ARCHIVED_SIGNAL_TYPE, signal_payload).ok();
        }
        // Goal Comments
        DirectMessage::GoalCommentNotification(signal_payload) => {
            hdk::emit_signal(GOAL_COMMENT_SIGNAL_TYPE, signal_payload).ok();
        }
        DirectMessage::GoalCommentArchivedNotification(signal_payload) => {
            hdk::emit_signal(GOAL_COMMENT_ARCHIVED_SIGNAL_TYPE, signal_payload).ok();
        }
        // Goal Members
        DirectMessage::GoalMemberNotification(signal_payload) => {
            hdk::emit_signal(GOAL_MEMBER_SIGNAL_TYPE, signal_payload).ok();
        }
        DirectMessage::GoalMemberArchivedNotification(signal_payload) => {
            hdk::emit_signal(GOAL_MEMBER_ARCHIVED_SIGNAL_TYPE, signal_payload).ok();
        }
        // Goal Votes
        DirectMessage::GoalVoteNotification(signal_payload) => {
            hdk::emit_signal(GOAL_VOTE_SIGNAL_TYPE, signal_payload).ok();
        }
        DirectMessage::GoalVoteArchivedNotification(signal_payload) => {
            hdk::emit_signal(GOAL_VOTE_ARCHIVED_SIGNAL_TYPE, signal_payload).ok();
        }
    };
}

#[zome]
mod holo_acorn {

    #[init]
    pub fn init() {
        anchor::init()?;
        project::init()
    }

    #[validate_agent]
    pub fn validate_agent(validation_data: EntryValidationData<AgentId>) {
        Ok(())
    }

    #[receive]
    pub fn receive(from: Address, msg_json: JsonString) -> String {
        hdk::debug(format!("New direct message from: {:?}", from)).ok();
        let maybe_message: Result<DirectMessage, _> = JsonString::from_json(&msg_json).try_into();
        match maybe_message {
            Err(err) => format!("Err({})", err),
            Ok(message) => {
                signal_ui(&message);
                String::from("Ok")
            }
        }
    }

    #[entry_def]
    fn projectmeta_def() -> ValidatingEntryType {
        project::projectmeta_def()
    }

    #[entry_def]
    fn entry_point_def() -> ValidatingEntryType {
        project::entry_point_def()
    }

    #[entry_def]
    fn member_def() -> ValidatingEntryType {
        project::member_def()
    }

    #[entry_def]
    fn edge_def() -> ValidatingEntryType {
        project::edge_def()
    }

    #[entry_def]
    fn goal_def() -> ValidatingEntryType {
        project::goal_def()
    }
    #[entry_def]
    fn goal_comment_def() -> ValidatingEntryType {
        project::goal_comment_def()
    }

    #[entry_def]
    fn goal_member_def() -> ValidatingEntryType {
        project::goal_member_def()
    }
    #[entry_def]
    fn goal_vote_def() -> ValidatingEntryType {
        project::goal_vote_def()
    }

    // The anchor type. Anchors are app entries with type anchor. The value is how we find
    // the anchor again, for example, we create an anchor with app entry value 'goals' and
    // link all goals to that anchor.
    #[entry_def]
    fn anchor_def() -> ValidatingEntryType {
        anchor::anchor_def()
    }

    #[zome_fn("hc_public")]
    fn create_project_meta(projectmeta: ProjectMeta) -> ZomeApiResult<GetResponse<ProjectMeta>> {
        project::create_project_meta(projectmeta)
    }

    #[zome_fn("hc_public")]
    fn update_project_meta(projectmeta: ProjectMeta, address: Address) -> ZomeApiResult<GetResponse<ProjectMeta>> {
        project::update_project_meta(projectmeta, address)
    }

    #[zome_fn("hc_public")]
    fn fetch_project_meta() -> ZomeApiResult<GetResponse<ProjectMeta>> {
        project::fetch_project_meta()
    }

    #[zome_fn("hc_public")]
    fn create_entry_point(entry_point: EntryPoint) -> ZomeApiResult<EntryPointResponse> {
        project::create_entry_point(entry_point)
    }

    #[zome_fn("hc_public")]
    fn archive_entry_point(address: Address) -> ZomeApiResult<Address> {
        project::archive_entry_point(address)
    }

    #[zome_fn("hc_public")]
    fn fetch_entry_points() -> ZomeApiResult<Vec<EntryPointResponse>> {
        project::fetch_entry_points()
    }


    #[zome_fn("hc_public")]
    fn fetch_members() -> ZomeApiResult<Vec<Member>> {
        project::fetch_members()
    }

    #[zome_fn("hc_public")]
    fn history_of_goal(address: Address) -> ZomeApiResult<GetHistoryResponse> {
        project::history_of_goal(address)
    }

    #[zome_fn("hc_public")]
    fn create_goal(
        goal: Goal,
        maybe_parent_address: Option<Address>,
    ) -> ZomeApiResult<GoalMaybeWithEdge> {
        project::create_goal(goal, maybe_parent_address)
    }

    #[zome_fn("hc_public")]
    fn update_goal(goal: Goal, address: Address) -> ZomeApiResult<GetResponse<Goal>> {
        project::update_goal(goal, address)
    }
    #[zome_fn("hc_public")]
    fn update_goal_vote(
        goal_vote: GoalVote,
        address: Address,
    ) -> ZomeApiResult<GetResponse<GoalVote>> {
        project::update_goal_vote(goal_vote, address)
    }
    #[zome_fn("hc_public")]
    fn update_goal_comment(
        goal_comment: GoalComment,
        address: Address,
    ) -> ZomeApiResult<GetResponse<GoalComment>> {
        project::update_goal_comment(goal_comment, address)
    }

    #[zome_fn("hc_public")]
    fn create_edge(edge: Edge) -> ZomeApiResult<GetResponse<Edge>> {
        project::create_edge(edge)
    }
    #[zome_fn("hc_public")]
    fn fetch_goals() -> ZomeApiResult<Vec<GetResponse<Goal>>> {
        project::fetch_goals()
    }

    #[zome_fn("hc_public")]
    fn fetch_edges() -> ZomeApiResult<Vec<GetResponse<Edge>>> {
        project::fetch_edges()
    }

    #[zome_fn("hc_public")]
    fn archive_goal(address: Address) -> ZomeApiResult<ArchiveGoalResponse> {
        project::archive_goal(address)
    }

    #[zome_fn("hc_public")]
    fn archive_edge(address: Address) -> ZomeApiResult<Address> {
        project::archive_edge(address)
    }
    #[zome_fn("hc_public")]
    fn add_member_of_goal(goal_member: GoalMember) -> ZomeApiResult<GetResponse<GoalMember>> {
        project::add_member_of_goal(goal_member)
    }

    #[zome_fn("hc_public")]
    fn add_vote_of_goal(goal_vote: GoalVote) -> ZomeApiResult<GetResponse<GoalVote>> {
        project::add_vote_of_goal(goal_vote)
    }
    #[zome_fn("hc_public")]
    fn add_comment_of_goal(goal_comment: GoalComment) -> ZomeApiResult<GetResponse<GoalComment>> {
        project::add_comment_of_goal(goal_comment)
    }
    #[zome_fn("hc_public")]
    fn archive_members_of_goal(goal_address: Address) -> ZomeApiResult<Vec<Address>> {
        project::archive_members_of_goal(&goal_address)
    }
    #[zome_fn("hc_public")]
    fn archive_member_of_goal(address: Address) -> ZomeApiResult<Address> {
        project::archive_member_of_goal(address)
    }
    #[zome_fn("hc_public")]
    fn archive_vote_of_goal(address: Address) -> ZomeApiResult<Address> {
        project::archive_vote_of_goal(address)
    }
    #[zome_fn("hc_public")]
    fn archive_comment_of_goal(address: Address) -> ZomeApiResult<Address> {
        project::archive_comment_of_goal(address)
    }

    #[zome_fn("hc_public")]
    fn fetch_goal_members() -> ZomeApiResult<Vec<GetResponse<GoalMember>>> {
        project::fetch_goal_members()
    }
    #[zome_fn("hc_public")]
    fn fetch_goal_votes() -> ZomeApiResult<Vec<GetResponse<GoalVote>>> {
        project::fetch_goal_votes()
    }
    #[zome_fn("hc_public")]
    fn fetch_goal_comments() -> ZomeApiResult<Vec<GetResponse<GoalComment>>> {
        project::fetch_goal_comments()
    }
}
