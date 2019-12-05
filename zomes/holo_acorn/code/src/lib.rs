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

use hdk::holochain_core_types::{
    // agent::AgentId, dna::entry_types::Sharing, entry::Entry, link::LinkMatch,
    dna::entry_types::Sharing, entry::Entry, link::LinkMatch,
};
use hdk::{
    entry_definition::ValidatingEntryType,
    error::{ZomeApiResult},
    AGENT_ADDRESS,
    // AGENT_ADDRESS, AGENT_ID_STR,
};

use hdk::holochain_json_api::{
    error::JsonError,
    json::{default_to_json, JsonString},
};

use hdk::holochain_persistence_api::cas::content::{Address, AddressableContent};

use hdk_proc_macros::zome;

use serde::Serialize;
//use std::convert::{TryFrom};
// use std::convert::{TryFrom, TryInto};
use std::fmt::Debug;

// see https://developer.holochain.org/api/latest/hdk/ for info on using the hdk library

// a bit of profile info for an agent
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Profile {
    first_name: String,
    last_name: String,
    handle: String,
    avatar_url: String,
    address: String,
}

// a relationship between a Goal and an Agent
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct GoalMember {
    goal_address: Address,
    agent_address: Address,
    unix_timestamp: u128,
}
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct GoalVote {
    goal_address: Address,
    urgency:f32,
    importance:f32,
    impact:f32,
    effort:f32,
    agent_address: Address,
    unix_timestamp: u128,
}
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct GoalComment {
    goal_address: Address,
    content:String,
    agent_address: Address,
    unix_timestamp: u128,
}

// An edge. This is an arrow on the SoA Tree which directionally links
// two goals.
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Edge {
    parent_address: Address,
    child_address: Address,
}

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub enum Status {
    Uncertain,
    Incomplete,
    Complete,
    InReview
}

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub enum Hierarchy {
    Root,
    Trunk,
    Branch,
    Leaf
}
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct TimeFrame {
    from_date: u128,
    to_date: u128,
}

// A Goal Card. This is a card on the SoA Tree which can be small or non-small, complete or
// incomplete, certain or uncertain, and contains text content.
// user hash and unix timestamp are included to prevent hash collisions.
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Goal {
    content: String,
    user_hash: Address,
    unix_timestamp: u128,
    hierarchy: Hierarchy,
    status: Status,
    description:String,
    time_frame:Option<TimeFrame>
}
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct GoalMaybeWithEdge {
    goal: GetResponse<Goal>,
    maybe_edge: Option<GetResponse<Edge>>,
}

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct ArchiveGoalResponse {
    address: Address,
    archived_edges: Vec<Address>,
    archived_goal_members: Vec<Address>,
    archived_goal_votes: Vec<Address>,
    archived_goal_comments:Vec<Address>,
}

// The GetResponse struct allows our zome functions to return an entry along with its
// address so that Redux can know the address of goals and edges
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetResponse<T> {
    pub entry: T,
    pub address: Address,
}

impl<T: Into<JsonString> + Debug + Serialize> From<GetResponse<T>> for JsonString {
    fn from(u: GetResponse<T>) -> JsonString {
        default_to_json(u)
    }
}

#[zome]
mod holo_acorn {

    #[init]
    pub fn init() {
        // create anchor entries
        let goals_anchor_entry = Entry::App(
            "anchor".into(), // app entry type
            // app entry value. We'll use the value to specify what this anchor is for
            "goals".into(),
        );
        let edges_anchor_entry = Entry::App(
            "anchor".into(), // app entry type
            // app entry value. We'll use the value to specify what this anchor is for
            "edges".into(),
        );
        let goal_members_anchor_entry = Entry::App(
            "anchor".into(), // app entry type
            // app entry value. We'll use the value to specify what this anchor is for
            "goal_members".into(),
        );
        let goal_vote_anchor_entry = Entry::App(
            "anchor".into(), // app entry type
            // app entry value. We'll use the value to specify what this anchor is for
            "goal_votes".into(),
        );
        let goal_comment_anchor_entry = Entry::App(
            "anchor".into(), // app entry type
            // app entry value. We'll use the value to specify what this anchor is for
            "goal_comments".into(),
        );
        let agents_anchor_entry = Entry::App(
            "anchor".into(), // app entry type
            // app entry value. We'll use the value to specify what this anchor is for
            "agents".into(),
        );
        hdk::commit_entry(&goal_comment_anchor_entry)?;
        hdk::commit_entry(&goal_vote_anchor_entry)?;
        hdk::commit_entry(&goal_members_anchor_entry)?;
        hdk::commit_entry(&goals_anchor_entry)?;
        hdk::commit_entry(&edges_anchor_entry)?;
        hdk::commit_entry(&agents_anchor_entry)?;

        Ok(())
    }

    #[validate_agent]
    pub fn validate_agent(validation_data: EntryValidationData<AgentId>) {
        Ok(())
    }

    #[entry_def]
    fn profile_def() -> ValidatingEntryType {
        entry!(
            name: "profile",
            description: "this is an entry representing some profile info for an agent",
            sharing: Sharing::Public,
            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },
            validation: | _validation_data: hdk::EntryValidationData<Profile>| {
                Ok(())
            },
            links: [
                from!(
                    "%agent_id",
                    link_type: "agent->profile",
                    validation_package: || {
                        hdk::ValidationPackageDefinition::Entry
                    },
                    validation: | _validation_data: hdk::LinkValidationData| {
                        Ok(())
                    }
                )
            ]
        )
    }

    #[entry_def]
    fn edge_def() -> ValidatingEntryType {
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

    #[entry_def]
    fn goal_def() -> ValidatingEntryType {
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
    #[entry_def]
    fn goal_comment_def() -> ValidatingEntryType {
        entry!(
            name: "goal_comment",
            description: "this is an entry representing a goal",
            sharing: Sharing::Public,
            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },
            validation: | _validation_data: hdk::EntryValidationData<GoalComment>| {
                Ok(())
            }
        )
    }

    #[entry_def]
    fn goal_member_def() -> ValidatingEntryType {
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
    #[entry_def]
    fn goal_vote_def() -> ValidatingEntryType {
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

    // The anchor type. Anchors are app entries with type anchor. The value is how we find
    // the anchor again, for example, we create an anchor with app entry value 'goals' and
    // link all goals to that anchor.
    #[entry_def]
    fn anchor_def() -> ValidatingEntryType {
        entry!(
            name: "anchor",
            description: "this is an anchor entry that we can link other entries to so we can find them",
            sharing: Sharing::Public,
            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },
            validation: | _validation_data: hdk::EntryValidationData<String>| {
                Ok(())
            },
            links: [
                to!(
                    "profile",
                    link_type: "anchor->profiles",
                    validation_package: || {
                        hdk::ValidationPackageDefinition::Entry
                    },
                    validation: | _validation_data: hdk::LinkValidationData| {
                        Ok(())
                    }
                ),
                to!(
                    "goal",
                    link_type: "anchor->goal",
                    validation_package: || {
                        hdk::ValidationPackageDefinition::Entry
                    },
                    validation: | _validation_data: hdk::LinkValidationData| {
                        Ok(())
                    }
                ),
                to!(
                    "edge",
                    link_type: "anchor->edge",
                    validation_package: || {
                        hdk::ValidationPackageDefinition::Entry
                    },
                    validation: | _validation_data: hdk::LinkValidationData| {
                        Ok(())
                    }
                ),
                to!(
                    "goal_member",
                    link_type: "anchor->goal_member",
                    validation_package: || {
                        hdk::ValidationPackageDefinition::Entry
                    },
                    validation: | _validation_data: hdk::LinkValidationData| {
                        Ok(())
                    }
                ),
                to!(
                    "goal_vote",
                    link_type: "anchor->goal_vote",
                    validation_package: || {
                        hdk::ValidationPackageDefinition::Entry
                    },
                    validation: | _validation_data: hdk::LinkValidationData| {
                        Ok(())
                    }
                ),
                to!(
                    "goal_comment",
                    link_type: "anchor->goal_comment",
                    validation_package: || {
                        hdk::ValidationPackageDefinition::Entry
                    },
                    validation: | _validation_data: hdk::LinkValidationData| {
                        Ok(())
                    }
                )
            ]
        )
    }

    #[zome_fn("hc_public")]
    fn create_whoami(profile: Profile) -> ZomeApiResult<GetResponse<Profile>> {
      let agents_anchor_entry = Entry::App(
          "anchor".into(), // app entry type
          // app entry value. We'll use the value to specify what this anchor is for
          "agents".into(),
      );
      let profile_entry = Entry::App("profile".into(), profile.clone().into());
      let profile_address = hdk::commit_entry(&profile_entry)?;
      hdk::link_entries(
          &agents_anchor_entry.address(),
          &profile_address,
          "anchor->profiles",
          "",
      )?;
      hdk::link_entries(
          &AGENT_ADDRESS,
          &profile_address,
          "agent->profile",
          "",
      )?;
      Ok(GetResponse {
        entry: profile,
        address: profile_address
      })
    }

    #[zome_fn("hc_public")]
    fn update_whoami(profile: Profile, address: Address) -> ZomeApiResult<GetResponse<Profile>> {
      let profile_entry = Entry::App("profile".into(), profile.clone().into());
      hdk::update_entry(profile_entry, &address)?;
      Ok(GetResponse {
        entry: profile,
        address: address
      })
    }

    #[zome_fn("hc_public")]
    fn whoami() -> ZomeApiResult<Option<GetResponse<Profile>>> {
      match hdk::utils::get_links_and_load_type::<Profile>(
          &AGENT_ADDRESS,
          LinkMatch::Exactly("agent->profile"), // the link type to match
          LinkMatch::Any,
      )?.first() {
        Some(my_profile) => {
          let app_entry = Entry::App("profile".into(), my_profile.into());
          Ok(Some(GetResponse {
            entry: my_profile.clone(),
            address: app_entry.address()
          }))
        }
        None => Ok(None)
      }
    }

    #[zome_fn("hc_public")]
    fn fetch_agent_address() -> ZomeApiResult<Address> {
      Ok(AGENT_ADDRESS.clone())
    }

    #[zome_fn("hc_public")]
    fn fetch_agents() -> ZomeApiResult<Vec<Profile>> {
        let anchor_address = Entry::App(
            "anchor".into(), // app entry type
            // app entry value. We'll use the value to specify what this anchor is for
            "agents".into(),
        )
        .address();

        Ok(
            // return all the Profile objects from the entries linked to the profiles anchor (drop entries with wrong type)
            hdk::utils::get_links_and_load_type(
                &anchor_address,
                LinkMatch::Exactly("anchor->profiles"), // the link type to match
                LinkMatch::Any,
            )?
        )
    }
    
    #[zome_fn("hc_public")]
    fn create_goal(
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
        
        // format the response as a GetResponse
        Ok(GoalMaybeWithEdge {
            goal: GetResponse {
                entry: goal,
                address: entry_address,
            },
            maybe_edge,
        })
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

    #[zome_fn("hc_public")]
    fn update_goal(goal: Goal, address: Address) -> ZomeApiResult<GetResponse<Goal>> {
        let app_entry = Entry::App("goal".into(), goal.clone().into());
        let _ = hdk::update_entry(app_entry, &address)?;

        // format the response as a GetResponse
        // pass the OLD address back and allow the UI to continue to use it
        Ok(GetResponse {
            entry: goal,
            address,
        })
    }
    #[zome_fn("hc_public")]
    fn update_goal_vote(goal_vote:GoalVote , address: Address) -> ZomeApiResult<GetResponse<GoalVote>> {
        let app_entry = Entry::App("goal_vote".into(), goal_vote.clone().into());
        let _ = hdk::update_entry(app_entry, &address)?;

        // format the response as a GetResponse
        // pass the OLD address back and allow the UI to continue to use it
        Ok(GetResponse {
            entry: goal_vote,
            address,
        })
    }
     #[zome_fn("hc_public")]
    fn update_goal_comment(goal_comment:GoalComment , address: Address) -> ZomeApiResult<GetResponse<GoalComment>> {
        let app_entry = Entry::App("goal_comment".into(), goal_comment.clone().into());
        let _ = hdk::update_entry(app_entry, &address)?;

        // format the response as a GetResponse
        // pass the OLD address back and allow the UI to continue to use it
        Ok(GetResponse {
            entry: goal_comment,
            address,
        })
    }
    

    #[zome_fn("hc_public")]
    fn create_edge(edge: Edge) -> ZomeApiResult<GetResponse<Edge>> {
        let entry_address = inner_create_edge(&edge)?;
        Ok(GetResponse {
            entry: edge,
            address: entry_address,
        })
    }
     #[zome_fn("hc_public")]
    fn fetch_goals() -> ZomeApiResult<Vec<GetResponse<Goal>>> {
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
            .map(|address: Address| match hdk::utils::get_as_type(address.clone()) {
                Ok(goal) =>Ok(GetResponse {
                                entry: goal,
                                address,
                            }),
                        
                        Err(e) => Err(e)
            })
            .filter_map(Result::ok)
            .collect(),
        )
    }

    fn inner_fetch_goal_members() -> ZomeApiResult<Vec<GetResponse<GoalMember>>> {
        // set up the anchor entry and compute its address
        let anchor_address = Entry::App(
            "anchor".into(), // app entry type
            "goal_members".into(),  // app entry value
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
            "anchor".into(), // app entry type
            "goal_votes".into(),  // app entry value
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
            "anchor".into(), // app entry type
            "goal_comments".into(),  // app entry value
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

    #[zome_fn("hc_public")]
    fn fetch_edges() -> ZomeApiResult<Vec<GetResponse<Edge>>> {
        inner_fetch_edges()
    }

    #[zome_fn("hc_public")]
    fn archive_goal(address: Address) -> ZomeApiResult<ArchiveGoalResponse> {
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

        let archived_goal_members = inner_fetch_goal_members()?
            .into_iter()
            .filter(|get_response: &GetResponse<GoalMember>| {
                // check whether the parent_address or child_address is equal to the given address.
                // If so, the edge is connected to the goal being archived.
                get_response.entry.goal_address == address
            })
            .map(|get_response: GetResponse<GoalMember>| {
                let goal_member_address = get_response.address;
                // archive the edge with this address
                match hdk::remove_entry(&goal_member_address) {
                    Ok(_) => Ok(goal_member_address),
                    Err(e) => Err(e),
                }
            })
            // filter out errors
            .filter_map(Result::ok)
            .collect(); // returns vec of the goal_member addresses which were removed
        let archived_goal_votes= inner_fetch_goal_votes()?
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
             let archived_goal_comments= inner_fetch_goal_comments()?
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
            .collect();  // returns vec of the goal_member addresses which were removed
        // return the address of the archived goal for the UI to use
        Ok(ArchiveGoalResponse {
            address,
            archived_edges,
            archived_goal_members,
            archived_goal_votes,
            archived_goal_comments,
        })
    }

    #[zome_fn("hc_public")]
    fn archive_edge(address: Address) -> ZomeApiResult<Address> {
        hdk::remove_entry(&address)?;
        Ok(address)
    }
    #[zome_fn("hc_public")]
    fn add_member_of_goal(goal_member: GoalMember) -> ZomeApiResult<GetResponse<GoalMember>> {
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

        Ok(GetResponse {
            entry: goal_member,
            address: entry_address,
        })
    }

    #[zome_fn("hc_public")]
    fn add_vote_of_goal(goal_vote: GoalVote) -> ZomeApiResult<GetResponse<GoalVote>> {
        let app_entry = Entry::App("goal_vote".into(), goal_vote.clone().into());
        let entry_address = hdk::commit_entry(&app_entry)?;

        // link new edge to the edges anchor
        let anchor_address = Entry::App(
            "anchor".into(),       // app entry type
            "goal_votes".into(), // app entry value
        )
        .address();

        hdk::link_entries(
            &anchor_address,
            &app_entry.address(),
            "anchor->goal_vote",
            "",
        )?;

        Ok(GetResponse {
            entry: goal_vote,
            address: entry_address,
        })
    }
    #[zome_fn("hc_public")]
    fn add_comment_of_goal(goal_comment: GoalComment) -> ZomeApiResult<GetResponse<GoalComment>> {
        let app_entry = Entry::App("goal_comment".into(), goal_comment.clone().into());
        let entry_address = hdk::commit_entry(&app_entry)?;

        // link new edge to the edges anchor
        let anchor_address = Entry::App(
            "anchor".into(),       // app entry type
            "goal_comments".into(), // app entry value
        )
        .address();

        hdk::link_entries(
            &anchor_address,
            &app_entry.address(),
            "anchor->goal_comment",
            "",
        )?;

        Ok(GetResponse {
            entry: goal_comment,
            address: entry_address,
        })
    }
    #[zome_fn("hc_public")]
    fn archive_member_of_goal(address: Address) -> ZomeApiResult<Address> {
        hdk::remove_entry(&address)?;
        Ok(address)
    }
     #[zome_fn("hc_public")]
    fn archive_vote_of_goal(address: Address) -> ZomeApiResult<Address> {
        hdk::remove_entry(&address)?;
        Ok(address)
    }
     #[zome_fn("hc_public")]
    fn archive_comment_of_goal(address: Address) -> ZomeApiResult<Address> {
        hdk::remove_entry(&address)?;
        Ok(address)
    }


    #[zome_fn("hc_public")]
    fn fetch_goal_members() -> ZomeApiResult<Vec<GetResponse<GoalMember>>> {
        inner_fetch_goal_members()
    }
    #[zome_fn("hc_public")]
    fn fetch_goal_votes() -> ZomeApiResult<Vec<GetResponse<GoalVote>>> {
        inner_fetch_goal_votes()
    }
    #[zome_fn("hc_public")]
    fn fetch_goal_comments() -> ZomeApiResult<Vec<GetResponse<GoalComment>>> {
        inner_fetch_goal_comments()
    }
}