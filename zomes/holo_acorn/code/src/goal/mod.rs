extern crate hdk;
extern crate hdk_proc_macros;
extern crate serde;

extern crate serde_derive;
extern crate serde_json;

extern crate holochain_json_derive;

use hdk::{
    entry_definition::ValidatingEntryType,
    error::{ZomeApiError, ZomeApiResult},
    holochain_core_types::{
        // agent::AgentId, dna::entry_types::Sharing, entry::Entry, link::LinkMatch,
        dna::entry_types::Sharing,
        entry::Entry,
        link::LinkMatch,
    },
    holochain_json_api::{error::JsonError, json::JsonString},
    holochain_persistence_api::cas::content::{Address, AddressableContent},
    prelude::Entry::App,
    // AGENT_ADDRESS, AGENT_ID_STR,
    AGENT_ADDRESS,
};

use crate::profile::{notify_all, GetResponse};
use crate::{
    DirectMessage, EntryArchivedSignalPayload, GoalArchivedSignalPayload, GoalCommentSignalPayload,
    GoalMaybeWithEdgeSignalPayload, GoalMemberSignalPayload, GoalVoteSignalPayload,
};

// a bit of profile info for an agent
// a relationship between a Goal and an Agent
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone, PartialEq)]
pub struct GoalMember {
    goal_address: Address,
    agent_address: Address,
    user_edit_hash: Option<Address>,
    unix_timestamp: u128,
}
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone, PartialEq)]
pub struct GoalVote {
    goal_address: Address,
    urgency: f32,
    importance: f32,
    impact: f32,
    effort: f32,
    agent_address: Address,
    unix_timestamp: u128,
}
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone, PartialEq)]
pub struct GoalComment {
    goal_address: Address,
    content: String,
    agent_address: Address,
    unix_timestamp: u128,
}

// An edge. This is an arrow on the SoA Tree which directionally links
// two goals.
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone, PartialEq)]
pub struct Edge {
    parent_address: Address,
    child_address: Address,
}

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone, PartialEq)]
pub enum Status {
    Uncertain,
    Incomplete,
    InProcess,
    Complete,
    InReview,
}

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone, PartialEq)]
pub enum Hierarchy {
    Root,
    Trunk,
    Branch,
    Leaf,
    NoHierarchy,
}
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone, PartialEq)]
pub struct TimeFrame {
    from_date: u128,
    to_date: u128,
}

// A Goal Card. This is a card on the SoA Tree which can be small or non-small, complete or
// incomplete, certain or uncertain, and contains text content.
// user hash and unix timestamp are included to prevent hash collisions.
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone, PartialEq)]
pub struct Goal {
    content: String,
    user_hash: Address,
    user_edit_hash: Option<Address>,
    timestamp_created: u128,
    timestamp_updated: Option<u128>,
    hierarchy: Hierarchy,
    status: Status,
    tags: Option<Vec<String>>,
    description: String,
    time_frame: Option<TimeFrame>,
}
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone, PartialEq)]
pub struct GoalMaybeWithEdge {
    goal: GetResponse<Goal>,
    maybe_edge: Option<GetResponse<Edge>>,
}

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone, PartialEq)]
pub struct ArchiveGoalResponse {
    address: Address,
    archived_edges: Vec<Address>,
    archived_goal_members: Vec<Address>,
    archived_goal_votes: Vec<Address>,
    archived_goal_comments: Vec<Address>,
}
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct GetHistoryResponse {
    entries: Vec<Goal>,
    members: Vec<Vec<GoalMember>>,
    address: Address,
}
//The GetResponse struct allows our zome functions to return an entry along with its
//address so that Redux can know the address of goals and edges
pub fn edge_def() -> ValidatingEntryType {
    entry!(
        name: "edge",
        description: "this is an entry representing a edge",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | _validation_data: hdk::EntryValidationData<Edge>| {
            Ok(())
        }
    )
}

pub fn goal_def() -> ValidatingEntryType {
    entry!(
        name: "goal",
        description: "this is an entry representing a goal",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | _validation_data: hdk::EntryValidationData<Goal>| {
             Ok(())
        }
    )
}
pub fn goal_comment_def() -> ValidatingEntryType {
    entry!(
        name: "goal_comment",
        description: "this is an entry representing a goal",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | validation_data: hdk::EntryValidationData<GoalComment>| {
            match validation_data{
                hdk::EntryValidationData::Create{entry,validation_data}=>{
                    let agent_address = &validation_data.sources()[0];
                    if entry.agent_address.to_string()!=agent_address.to_string() {
                        Err("only the same agent making a comment can make it in their name".into())
                    }else {Ok(())}
                },
                hdk::EntryValidationData::Modify{
                    new_entry,
                    old_entry,validation_data,..}=>{
                    let agent_address = &validation_data.sources()[0];

                    if new_entry.agent_address.to_string()!=agent_address.to_string()&& old_entry.agent_address.to_string()!=agent_address.to_string(){
                        Err("an agent can only update their own comment".into())
                    }else {Ok(())}
                },
                hdk::EntryValidationData::Delete{old_entry,validation_data,..}=>{
                    let agent_address = &validation_data.sources()[0];
                    if old_entry.agent_address.to_string()!=agent_address.to_string() {
                        Err("an agent can only delete their own comment".into())
                    }else {Ok(())}
                }
            }
        }
    )
}

pub fn goal_member_def() -> ValidatingEntryType {
    entry!(
        name: "goal_member",
        description: "this is an entry representing a connection from a Goal to an Agent",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | _validation_data: hdk::EntryValidationData<GoalMember>| {
            Ok(())
        }
    )
}
pub fn goal_vote_def() -> ValidatingEntryType {
    entry!(
        name: "goal_vote",
        description: "this is an entry representing a connection from a Goal to an Vote (priority)",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | _validation_data: hdk::EntryValidationData<GoalVote>| {
            Ok(())
        }
    )
}

fn notify_goal_maybe_with_edge(goal_maybe_with_edge: GoalMaybeWithEdge) -> ZomeApiResult<()> {
    let message = DirectMessage::GoalMaybeWithEdgeNotification(GoalMaybeWithEdgeSignalPayload {
        goal: goal_maybe_with_edge.clone(),
    });
    notify_all(message)
}

fn notify_goal_archived(archived: ArchiveGoalResponse) -> ZomeApiResult<()> {
    let message = DirectMessage::GoalArchivedNotification(GoalArchivedSignalPayload { archived });
    notify_all(message)
}

fn notify_goal_comment(goal_comment: GetResponse<GoalComment>) -> ZomeApiResult<()> {
    let message = DirectMessage::GoalCommentNotification(GoalCommentSignalPayload { goal_comment });
    notify_all(message)
}

fn notify_goal_member(goal_member: GetResponse<GoalMember>) -> ZomeApiResult<()> {
    let message = DirectMessage::GoalMemberNotification(GoalMemberSignalPayload { goal_member });
    notify_all(message)
}

fn notify_goal_vote(goal_vote: GetResponse<GoalVote>) -> ZomeApiResult<()> {
    let message = DirectMessage::GoalVoteNotification(GoalVoteSignalPayload { goal_vote });
    notify_all(message)
}

fn notify_goal_comment_archived(address: Address) -> ZomeApiResult<()> {
    let message =
        DirectMessage::GoalCommentArchivedNotification(EntryArchivedSignalPayload { address });
    notify_all(message)
}

fn notify_goal_member_archived(address: Address) -> ZomeApiResult<()> {
    let message =
        DirectMessage::GoalMemberArchivedNotification(EntryArchivedSignalPayload { address });
    notify_all(message)
}

fn notify_goal_vote_archived(address: Address) -> ZomeApiResult<()> {
    let message =
        DirectMessage::GoalVoteArchivedNotification(EntryArchivedSignalPayload { address });
    notify_all(message)
}

pub fn history_of_goal(address: Address) -> ZomeApiResult<GetHistoryResponse> {
    let anchor_address = Entry::App(
        "anchor".into(),       // app entry type
        "goal_members".into(), // app entry value
    )
    .address();
    // return all the Goal objects from the entries linked to the edge anchor (drop entries with wrong type)
    let members = hdk::get_links(
        &anchor_address,
        LinkMatch::Exactly("anchor->goal_member"), // the link type to match
        LinkMatch::Any,
    )?
    // scoop all these entries up into an array and return it
    .addresses()
    .into_iter()
    .map(|member_address: Address| {
        if let Ok(Some(entry_history)) = hdk::api::get_entry_history(&member_address) {
            Some(
                entry_history
                    .items
                    .into_iter()
                    .map(|item| {
                        if let Some(App(_, value_entry)) = item.entry {
                            match serde_json::from_str::<GoalMember>(&Into::<String>::into(
                                value_entry,
                            ))
                            .ok()
                            {
                                Some(goal_member) => {
                                    // filter down to only Goal Members that are associated with the requested Goal
                                    if goal_member.goal_address == address {
                                        Ok(goal_member)
                                    } else {
                                        Err(ZomeApiError::Internal("error".into()))
                                    }
                                }
                                None => Err(ZomeApiError::Internal("error".into())),
                            }
                        } else {
                            Err(ZomeApiError::Internal("error".into()))
                        }
                    })
                    .filter_map(Result::ok)
                    .collect(),
            )
        } else {
            None
        }
    })
    .filter_map(|op: Option<Vec<GoalMember>>| match op {
        Some(vec) => {
            if vec.len() > 0 {
                Some(vec)
            } else {
                None
            }
        }
        _ => None,
    })
    .collect();
    if let Ok(Some(entry_history)) = hdk::api::get_entry_history(&address) {
        Ok(GetHistoryResponse {
            entries: entry_history
                .items
                .into_iter()
                .map(|item| {
                    if let Some(App(_, value_entry)) = item.entry {
                        match serde_json::from_str::<Goal>(&Into::<String>::into(value_entry)).ok()
                        {
                            Some(goal) => Ok(goal),
                            None => Err(ZomeApiError::Internal("error".into())),
                        }
                    } else {
                        Err(ZomeApiError::Internal("error".into()))
                    }
                })
                .filter_map(Result::ok)
                .collect(),
            members: members,
            address: address,
        })
    } else {
        Err(ZomeApiError::Internal("error".into()))
    }
}

pub fn create_goal(
    goal: Goal,
    maybe_parent_address: Option<Address>,
) -> ZomeApiResult<GoalMaybeWithEdge> {
    let app_entry = Entry::App("goal".into(), goal.clone().into());
    let entry_address = hdk::commit_entry(&app_entry)?;

    // link new goal to the goals anchor
    let anchor_address = Entry::App(
        "anchor".into(), // app entry type
        "goals".into(),  // app entry value
    )
    .address();

    hdk::link_entries(&anchor_address, &app_entry.address(), "anchor->goal", "")?;

    // if a parent address was provided, link the goal with its parent
    let maybe_edge = match maybe_parent_address {
        Some(parent_address) => {
            let edge: Edge = Edge {
                parent_address: parent_address,
                child_address: entry_address.clone(),
            };
            let edge_address = inner_create_edge(&edge)?;
            Some(GetResponse {
                entry: edge,
                address: edge_address,
            })
        }
        None => None,
    };

    let goal_maybe_with_edge = GoalMaybeWithEdge {
        goal: GetResponse {
            entry: goal,
            address: entry_address,
        },
        maybe_edge,
    };
    notify_goal_maybe_with_edge(goal_maybe_with_edge.clone())?;
    // format the response as a GetResponse
    Ok(goal_maybe_with_edge)
}

fn inner_create_edge(edge: &Edge) -> ZomeApiResult<Address> {
    let app_entry = Entry::App("edge".into(), edge.clone().into());
    let entry_address = hdk::commit_entry(&app_entry)?;

    // link new edge to the edges anchor
    let anchor_address = Entry::App(
        "anchor".into(), // app entry type
        "edges".into(),  // app entry value
    )
    .address();

    hdk::link_entries(&anchor_address, &app_entry.address(), "anchor->edge", "")?;

    Ok(entry_address)
}

pub fn update_goal(goal: Goal, address: Address) -> ZomeApiResult<GetResponse<Goal>> {
    let mut goal = goal;
    let old_goal = hdk::utils::get_as_type::<Goal>(address.clone())?;
    if goal.timestamp_updated == None {
        goal.timestamp_updated = Some(goal.timestamp_created);
    }
    goal.timestamp_created = old_goal.timestamp_created;
    goal.user_edit_hash = Some(AGENT_ADDRESS.clone());
    let app_entry = Entry::App("goal".into(), goal.clone().into());
    let _ = hdk::update_entry(app_entry, &address)?;

    let goal = GetResponse {
        entry: goal,
        address,
    };
    // be careful of this GOTCHA...
    // notify_goal_maybe_with_edge
    // returns a different type response than this function
    // instead of the normal case where the types match
    let goal_maybe_with_edge = GoalMaybeWithEdge {
        goal: goal.clone(),
        maybe_edge: None,
    };
    notify_goal_maybe_with_edge(goal_maybe_with_edge)?;

    // format the response as a GetResponse
    // pass the OLD address back and allow the UI to continue to use it
    Ok(goal)
}

pub fn create_edge(edge: Edge) -> ZomeApiResult<GetResponse<Edge>> {
    let entry_address = inner_create_edge(&edge)?;
    Ok(GetResponse {
        entry: edge,
        address: entry_address,
    })
}
pub fn fetch_goals() -> ZomeApiResult<Vec<GetResponse<Goal>>> {
    // set up the anchor entry and compute its address
    let anchor_address = Entry::App(
        "anchor".into(), // app entry type
        "goals".into(),  // app entry value
    )
    .address();

    Ok(
        // return all the Goal objects from the entries linked to the edge anchor (drop entries with wrong type)
        hdk::get_links(
            &anchor_address,
            LinkMatch::Exactly("anchor->goal"), // the link type to match
            LinkMatch::Any,
        )?
        // scoop all these entries up into an array and return it
        .addresses()
        .into_iter()
        .map(
            |address: Address| match hdk::utils::get_as_type(address.clone()) {
                Ok(goal) => Ok(GetResponse {
                    entry: goal,
                    address,
                }),

                Err(e) => Err(e),
            },
        )
        .filter_map(Result::ok)
        .collect(),
    )
}

fn inner_fetch_goal_members() -> ZomeApiResult<Vec<GetResponse<GoalMember>>> {
    // set up the anchor entry and compute its address
    let anchor_address = Entry::App(
        "anchor".into(),       // app entry type
        "goal_members".into(), // app entry value
    )
    .address();

    Ok(
        // return all the GoalMember objects from the entries linked to the goal_members anchor (drop entries with wrong type)
        hdk::utils::get_links_and_load_type(
            &anchor_address,
            LinkMatch::Exactly("anchor->goal_member"), // the link type to match
            LinkMatch::Any,
        )?
        .into_iter()
        .map(|goal_member: GoalMember| {
            // re-create the goal_member entry to find its address
            let address = Entry::App("goal_member".into(), goal_member.clone().into()).address();
            // return a response structs with the edge and its address
            GetResponse {
                entry: goal_member,
                address,
            }
        })
        .collect(),
    )
}
fn inner_fetch_goal_votes() -> ZomeApiResult<Vec<GetResponse<GoalVote>>> {
    // set up the anchor entry and compute its address
    let anchor_address = Entry::App(
        "anchor".into(),     // app entry type
        "goal_votes".into(), // app entry value
    )
    .address();

    Ok(
        // return all the GoalMember objects from the entries linked to the goal_members anchor (drop entries with wrong type)
        hdk::utils::get_links_and_load_type(
            &anchor_address,
            LinkMatch::Exactly("anchor->goal_vote"), // the link type to match
            LinkMatch::Any,
        )?
        .into_iter()
        .map(|goal_vote: GoalVote| {
            // re-create the goal_member entry to find its address
            let address = Entry::App("goal_vote".into(), goal_vote.clone().into()).address();
            // return a response structs with the edge and its address
            GetResponse {
                entry: goal_vote,
                address,
            }
        })
        .collect(),
    )
}
fn inner_fetch_goal_comments() -> ZomeApiResult<Vec<GetResponse<GoalComment>>> {
    // set up the anchor entry and compute its address
    let anchor_address = Entry::App(
        "anchor".into(),        // app entry type
        "goal_comments".into(), // app entry value
    )
    .address();

    Ok(
        // return all the GoalMember objects from the entries linked to the goal_members anchor (drop entries with wrong type)
        hdk::utils::get_links_and_load_type(
            &anchor_address,
            LinkMatch::Exactly("anchor->goal_comment"), // the link type to match
            LinkMatch::Any,
        )?
        .into_iter()
        .map(|goal_comment: GoalComment| {
            // re-create the goal_member entry to find its address
            let address = Entry::App("goal_comment".into(), goal_comment.clone().into()).address();
            // return a response structs with the edge and its address
            GetResponse {
                entry: goal_comment,
                address,
            }
        })
        .collect(),
    )
}
fn inner_fetch_edges() -> ZomeApiResult<Vec<GetResponse<Edge>>> {
    // set up the anchor entry and compute its address
    let anchor_address = Entry::App(
        "anchor".into(), // app entry type
        "edges".into(),  // app entry value
    )
    .address();

    Ok(
        // return all the Edge objects from the entries linked to the edge anchor (drop entries with wrong type)
        hdk::utils::get_links_and_load_type(
            &anchor_address,
            LinkMatch::Exactly("anchor->edge"), // the link type to match
            LinkMatch::Any,
        )?
        .into_iter()
        .map(|edge: Edge| {
            // re-create the edge entry to find its address
            let address = Entry::App("edge".into(), edge.clone().into()).address();
            // return a response structs with the edge and its address
            GetResponse {
                entry: edge,
                address,
            }
        })
        .collect(),
    )
}

pub fn fetch_edges() -> ZomeApiResult<Vec<GetResponse<Edge>>> {
    inner_fetch_edges()
}

pub fn archive_goal(address: Address) -> ZomeApiResult<ArchiveGoalResponse> {
    // commit the removeEntry. Returns the address of the removeEntry
    hdk::remove_entry(&address)?;

    let archived_edges = inner_fetch_edges()?
        .into_iter()
        .filter(|get_response: &GetResponse<Edge>| {
            // check whether the parent_address or child_address is equal to the given address.
            // If so, the edge is connected to the goal being archived.
            get_response.entry.child_address == address
                || get_response.entry.parent_address == address
        })
        .map(|get_response: GetResponse<Edge>| {
            let edge_address = get_response.address;
            // archive the edge with this address
            match hdk::remove_entry(&edge_address) {
                Ok(_) => Ok(edge_address),
                Err(e) => Err(e),
            }
        })
        // filter out errors
        .filter_map(Result::ok)
        .collect(); // returns vec of the edge addresses which were removed

    let archived_goal_members = archive_members_of_goal(&address)?; // returns vec of the goal_member addresses which were removed
    let archived_goal_votes = inner_fetch_goal_votes()?
        .into_iter()
        .filter(|get_response: &GetResponse<GoalVote>| {
            // check whether the parent_address or child_address is equal to the given address.
            // If so, the edge is connected to the goal being archived.
            get_response.entry.goal_address == address
        })
        .map(|get_response: GetResponse<GoalVote>| {
            let goal_vote_address = get_response.address;
            // archive the edge with this address
            match hdk::remove_entry(&goal_vote_address) {
                Ok(_) => Ok(goal_vote_address),
                Err(e) => Err(e),
            }
        })
        // filter out errors
        .filter_map(Result::ok)
        .collect();
    let archived_goal_comments = inner_fetch_goal_comments()?
        .into_iter()
        .filter(|get_response: &GetResponse<GoalComment>| {
            // check whether the parent_address or child_address is equal to the given address.
            // If so, the edge is connected to the goal being archived.
            get_response.entry.goal_address == address
        })
        .map(|get_response: GetResponse<GoalComment>| {
            let goal_comment_address = get_response.address;
            // archive the edge with this address
            match hdk::remove_entry(&goal_comment_address) {
                Ok(_) => Ok(goal_comment_address),
                Err(e) => Err(e),
            }
        })
        // filter out errors
        .filter_map(Result::ok)
        .collect(); // returns vec of the goal_member addresses which were removed
                    // return the address of the archived goal for the UI to use

    let archive_response = ArchiveGoalResponse {
        address,
        archived_edges,
        archived_goal_members,
        archived_goal_votes,
        archived_goal_comments,
    };
    notify_goal_archived(archive_response.clone())?;

    Ok(archive_response)
}

pub fn archive_edge(address: Address) -> ZomeApiResult<Address> {
    hdk::remove_entry(&address)?;
    Ok(address)
}

pub fn add_member_of_goal(goal_member: GoalMember) -> ZomeApiResult<GetResponse<GoalMember>> {
    let mut goal_member = goal_member;
    goal_member.user_edit_hash = Some(AGENT_ADDRESS.clone());
    let app_entry = Entry::App("goal_member".into(), goal_member.clone().into());
    let entry_address = hdk::commit_entry(&app_entry)?;

    // link new edge to the edges anchor
    let anchor_address = Entry::App(
        "anchor".into(),       // app entry type
        "goal_members".into(), // app entry value
    )
    .address();

    hdk::link_entries(
        &anchor_address,
        &app_entry.address(),
        "anchor->goal_member",
        "",
    )?;

    let get_response = GetResponse {
        entry: goal_member,
        address: entry_address,
    };
    notify_goal_member(get_response.clone())?;

    Ok(get_response)
}

pub fn add_vote_of_goal(goal_vote: GoalVote) -> ZomeApiResult<GetResponse<GoalVote>> {
    let app_entry = Entry::App("goal_vote".into(), goal_vote.clone().into());
    let entry_address = hdk::commit_entry(&app_entry)?;

    // link new edge to the edges anchor
    let anchor_address = Entry::App(
        "anchor".into(),     // app entry type
        "goal_votes".into(), // app entry value
    )
    .address();

    hdk::link_entries(
        &anchor_address,
        &app_entry.address(),
        "anchor->goal_vote",
        "",
    )?;

    let get_response = GetResponse {
        entry: goal_vote,
        address: entry_address,
    };
    notify_goal_vote(get_response.clone())?;

    Ok(get_response)
}

pub fn update_goal_vote(
    goal_vote: GoalVote,
    address: Address,
) -> ZomeApiResult<GetResponse<GoalVote>> {
    let app_entry = Entry::App("goal_vote".into(), goal_vote.clone().into());
    let _ = hdk::update_entry(app_entry, &address)?;

    // format the response as a GetResponse
    // pass the OLD address back and allow the UI to continue to use it
    let get_response = GetResponse {
        entry: goal_vote,
        address: address,
    };
    notify_goal_vote(get_response.clone())?;

    Ok(get_response)
}

pub fn add_comment_of_goal(goal_comment: GoalComment) -> ZomeApiResult<GetResponse<GoalComment>> {
    let app_entry = Entry::App("goal_comment".into(), goal_comment.clone().into());
    let entry_address = hdk::commit_entry(&app_entry)?;

    // link new edge to the edges anchor
    let anchor_address = Entry::App(
        "anchor".into(),        // app entry type
        "goal_comments".into(), // app entry value
    )
    .address();

    hdk::link_entries(
        &anchor_address,
        &app_entry.address(),
        "anchor->goal_comment",
        "",
    )?;

    let get_response = GetResponse {
        entry: goal_comment,
        address: entry_address,
    };
    notify_goal_comment(get_response.clone())?;

    Ok(get_response)
}

pub fn update_goal_comment(
    goal_comment: GoalComment,
    address: Address,
) -> ZomeApiResult<GetResponse<GoalComment>> {
    let app_entry = Entry::App("goal_comment".into(), goal_comment.clone().into());
    let _ = hdk::update_entry(app_entry, &address)?;

    // format the response as a GetResponse
    // pass the OLD address back and allow the UI to continue to use it
    let get_response = GetResponse {
        entry: goal_comment,
        address,
    };
    notify_goal_comment(get_response.clone())?;

    Ok(get_response)
}

pub fn archive_members_of_goal(address: &Address) -> ZomeApiResult<Vec<Address>> {
    inner_fetch_goal_members()?
        .into_iter()
        .filter(|get_response: &GetResponse<GoalMember>| {
            // check whether the parent_address or child_address is equal to the given address.
            // If so, the edge is connected to the goal being archived.
            get_response.entry.goal_address == *address
        })
        .map(|get_response: GetResponse<GoalMember>| {
            let goal_member_address = get_response.address;
            // archive the edge with this address
            match hdk::remove_entry(&goal_member_address) {
                Ok(_) => {
                    notify_goal_member_archived(goal_member_address.clone())?;
                    Ok(goal_member_address)
                }
                Err(e) => Err(e),
            }
        })
        // filter out errors
        .collect()
}

pub fn archive_member_of_goal(address: Address) -> ZomeApiResult<Address> {
    hdk::remove_entry(&address)?;
    notify_goal_member_archived(address.clone())?;
    Ok(address)
}

pub fn archive_vote_of_goal(address: Address) -> ZomeApiResult<Address> {
    hdk::remove_entry(&address)?;
    notify_goal_vote_archived(address.clone())?;
    Ok(address)
}

pub fn archive_comment_of_goal(address: Address) -> ZomeApiResult<Address> {
    hdk::remove_entry(&address)?;
    notify_goal_comment_archived(address.clone())?;
    Ok(address)
}

pub fn fetch_goal_members() -> ZomeApiResult<Vec<GetResponse<GoalMember>>> {
    inner_fetch_goal_members()
}
pub fn fetch_goal_votes() -> ZomeApiResult<Vec<GetResponse<GoalVote>>> {
    inner_fetch_goal_votes()
}
pub fn fetch_goal_comments() -> ZomeApiResult<Vec<GetResponse<GoalComment>>> {
    inner_fetch_goal_comments()
}
