use dna_help::{
    fetch_links,
    signal_peers,
    WrappedAgentPubKey,
};
use hdk3::prelude::*;

mod project;

use project::{
    edge::{Edge, EdgeSignal, EDGE_PATH},
    entry_point::{EntryPoint, EntryPointSignal, ENTRY_POINT_PATH},
    goal::{ArchiveGoalFullySignal, Goal, GoalSignal, GoalWithEdgeSignal, GOAL_PATH},
    goal_comment::{GoalComment, GoalCommentSignal, GOAL_COMMENT_PATH},
    goal_member::{GoalMember, GoalMemberSignal, GOAL_MEMBER_PATH},
    goal_vote::{GoalVote, GoalVoteSignal, GOAL_VOTE_PATH},
    member::{Member, MemberSignal, MEMBER_PATH},
    project_meta::{ProjectMeta, ProjectMetaSignal, PROJECT_META_PATH},
};

#[hdk_extern]
fn init(_: ()) -> ExternResult<InitCallbackResult> {
    Path::from(EDGE_PATH).ensure()?;
    Path::from(ENTRY_POINT_PATH).ensure()?;
    Path::from(GOAL_PATH).ensure()?;
    Path::from(GOAL_COMMENT_PATH).ensure()?;
    Path::from(GOAL_MEMBER_PATH).ensure()?;
    Path::from(GOAL_VOTE_PATH).ensure()?;
    Path::from(PROJECT_META_PATH).ensure()?;
    Path::from(MEMBER_PATH).ensure()?;

    // while joining, list me in the list of members
    // so that all peers can become aware of the new presence
    let member_path_address = Path::from(MEMBER_PATH).hash()?;
    let member = Member {
        address: WrappedAgentPubKey(agent_info()?.agent_initial_pubkey),
    };
    // send update to peers alerting them that you joined
    // don't fail if something goes wrong here
    let _ = signal_peers(&MemberSignal::new(member.clone()), get_peers);

    // now create the link, so that we don't see ourselves in the get_peers list
    create_entry(&member)?;
    let member_entry_hash = hash_entry(&member)?;
    create_link(member_path_address, member_entry_hash, ())?;


    Ok(InitCallbackResult::Pass)
}

entry_defs!(
    Path::entry_def(),
    Edge::entry_def(),
    EntryPoint::entry_def(),
    Goal::entry_def(),
    GoalComment::entry_def(),
    GoalMember::entry_def(),
    GoalVote::entry_def(),
    Member::entry_def(),
    ProjectMeta::entry_def()
);

/*
SIGNALS
*/

#[derive(Debug, Serialize, Deserialize, SerializedBytes)]
// untagged because the useful tagging is done internally on the *Signal objects
#[serde(untagged)]
pub enum SignalType {
    Edge(EdgeSignal),
    EntryPoint(EntryPointSignal),
    Goal(GoalSignal),
    // custom signal type for a goal_with_edge
    // this is because it's important to the UI to receive both
    // the new goal, and the edge, at the same moment
    GoalWithEdge(GoalWithEdgeSignal),
    // custom signal type for goal_fully_archived
    // this is because it's important to the UI to receive
    // both the archived goal, and eveyrthing connected to it that
    // was archived at the same time
    ArchiveGoalFully(ArchiveGoalFullySignal),
    GoalComment(GoalCommentSignal),
    GoalMember(GoalMemberSignal),
    GoalVote(GoalVoteSignal),
    Member(MemberSignal),
    ProjectMeta(ProjectMetaSignal),
}

// used to get addresses of agents to send signals to
pub fn get_peers() -> ExternResult<Vec<AgentPubKey>> {
    let path_hash = Path::from(MEMBER_PATH).hash()?;
    let entries = fetch_links::<Member, Member>(path_hash)?;
    let agent_info = agent_info()?;
    debug!(format!("PEER ENTRIES {:?}", entries));
    Ok(entries
        .into_iter()
        // eliminate yourself as a peer
        .filter(|x| x.address.0 != agent_info.agent_initial_pubkey)
        .map(|x| x.address.0)
        .collect::<Vec<AgentPubKey>>())
}

// receiver (and forward to UI)
#[hdk_extern]
pub fn receive_signal(signal: SignalType) -> ExternResult<()> {
    match emit_signal(&signal) {
        Ok(_) => Ok(()),
        Err(_) => Err(HdkError::SerializedBytes(SerializedBytesError::ToBytes(
            "couldnt convert to bytes to send as signal".to_string(),
        ))),
    }
}
