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
    // AGENT_ADDRESS, AGENT_ID_STR,
    AGENT_ADDRESS,
};

use hdk::holochain_persistence_api::cas::content::Address;

use hdk_proc_macros::zome;

//use std::convert::{TryFrom};
// use std::convert::{TryFrom, TryInto};
mod profile;
use crate::profile::GetResponse;
use crate::profile::Profile;
mod anchor;
mod goal;

use goal::{
    ArchiveGoalResponse, Edge, GetHistoryResponse, Goal, GoalComment, GoalMaybeWithEdge,
    GoalMember, GoalVote,
};
//The GetResponse struct allows our zome functions to return an entry along with its
//address so that Redux can know the address of goals and edges

#[zome]
mod holo_acorn {

    #[init]
    pub fn init() {
        anchor::init()
    }

    #[validate_agent]
    pub fn validate_agent(validation_data: EntryValidationData<AgentId>) {
        Ok(())
    }

    #[entry_def]
    fn profile_def() -> ValidatingEntryType {
        profile::profile_def()
    }

    #[entry_def]
    fn edge_def() -> ValidatingEntryType {
        goal::edge_def()
    }

    #[entry_def]
    fn goal_def() -> ValidatingEntryType {
        goal::goal_def()
    }
    #[entry_def]
    fn goal_comment_def() -> ValidatingEntryType {
        goal::goal_comment_def()
    }

    #[entry_def]
    fn goal_member_def() -> ValidatingEntryType {
        goal::goal_member_def()
    }
    #[entry_def]
    fn goal_vote_def() -> ValidatingEntryType {
        goal::goal_vote_def()
    }

    // The anchor type. Anchors are app entries with type anchor. The value is how we find
    // the anchor again, for example, we create an anchor with app entry value 'goals' and
    // link all goals to that anchor.
    #[entry_def]
    fn anchor_def() -> ValidatingEntryType {
        anchor::anchor_def()
    }

    #[zome_fn("hc_public")]
    fn create_whoami(profile: Profile) -> ZomeApiResult<GetResponse<Profile>> {
        profile::create_whoami(profile)
    }

    #[zome_fn("hc_public")]
    fn update_whoami(profile: Profile, address: Address) -> ZomeApiResult<GetResponse<Profile>> {
        profile::update_whoami(profile, address)
    }

    #[zome_fn("hc_public")]
    fn whoami() -> ZomeApiResult<Option<GetResponse<Profile>>> {
        profile::whoami()
    }

    #[zome_fn("hc_public")]
    fn fetch_agent_address() -> ZomeApiResult<Address> {
        Ok(AGENT_ADDRESS.clone())
    }
    #[zome_fn("hc_public")]
    fn history_of_goal(address: Address) -> ZomeApiResult<GetHistoryResponse> {
        goal::history_of_goal(address)
    }
    #[zome_fn("hc_public")]
    fn fetch_agents() -> ZomeApiResult<Vec<Profile>> {
        profile::fetch_agents()
    }

    #[zome_fn("hc_public")]
    fn create_goal(
        goal: Goal,
        maybe_parent_address: Option<Address>,
    ) -> ZomeApiResult<GoalMaybeWithEdge> {
        goal::create_goal(goal, maybe_parent_address)
    }

    #[zome_fn("hc_public")]
    fn update_goal(goal: Goal, address: Address) -> ZomeApiResult<GetResponse<Goal>> {
        goal::update_goal(goal, address)
    }
    #[zome_fn("hc_public")]
    fn update_goal_vote(
        goal_vote: GoalVote,
        address: Address,
    ) -> ZomeApiResult<GetResponse<GoalVote>> {
        goal::update_goal_vote(goal_vote, address)
    }
    #[zome_fn("hc_public")]
    fn update_goal_comment(
        goal_comment: GoalComment,
        address: Address,
    ) -> ZomeApiResult<GetResponse<GoalComment>> {
        goal::update_goal_comment(goal_comment, address)
    }

    #[zome_fn("hc_public")]
    fn create_edge(edge: Edge) -> ZomeApiResult<GetResponse<Edge>> {
        goal::create_edge(edge)
    }
    #[zome_fn("hc_public")]
    fn fetch_goals() -> ZomeApiResult<Vec<GetResponse<Goal>>> {
        goal::fetch_goals()
    }

    #[zome_fn("hc_public")]
    fn fetch_edges() -> ZomeApiResult<Vec<GetResponse<Edge>>> {
        goal::fetch_edges()
    }

    #[zome_fn("hc_public")]
    fn archive_goal(address: Address) -> ZomeApiResult<ArchiveGoalResponse> {
        goal::archive_goal(address)
    }

    #[zome_fn("hc_public")]
    fn archive_edge(address: Address) -> ZomeApiResult<Address> {
        goal::archive_edge(address)
    }
    #[zome_fn("hc_public")]
    fn add_member_of_goal(goal_member: GoalMember) -> ZomeApiResult<GetResponse<GoalMember>> {
        goal::add_member_of_goal(goal_member)
    }

    #[zome_fn("hc_public")]
    fn add_vote_of_goal(goal_vote: GoalVote) -> ZomeApiResult<GetResponse<GoalVote>> {
        goal::add_vote_of_goal(goal_vote)
    }
    #[zome_fn("hc_public")]
    fn add_comment_of_goal(goal_comment: GoalComment) -> ZomeApiResult<GetResponse<GoalComment>> {
        goal::add_comment_of_goal(goal_comment)
    }
    #[zome_fn("hc_public")]
    fn archive_member_of_goal(address: Address) -> ZomeApiResult<Address> {
        goal::archive_member_of_goal(address)
    }
    #[zome_fn("hc_public")]
    fn archive_vote_of_goal(address: Address) -> ZomeApiResult<Address> {
        goal::archive_vote_of_goal(address)
    }
    #[zome_fn("hc_public")]
    fn archive_comment_of_goal(address: Address) -> ZomeApiResult<Address> {
        goal::archive_comment_of_goal(address)
    }

    #[zome_fn("hc_public")]
    fn fetch_goal_members() -> ZomeApiResult<Vec<GetResponse<GoalMember>>> {
        goal::fetch_goal_members()
    }
    #[zome_fn("hc_public")]
    fn fetch_goal_votes() -> ZomeApiResult<Vec<GetResponse<GoalVote>>> {
        goal::fetch_goal_votes()
    }
    #[zome_fn("hc_public")]
    fn fetch_goal_comments() -> ZomeApiResult<Vec<GetResponse<GoalComment>>> {
        goal::fetch_goal_comments()
    }
}
