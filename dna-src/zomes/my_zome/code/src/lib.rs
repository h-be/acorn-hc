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
};
use hdk::holochain_core_types::{
    entry::Entry,
    dna::entry_types::Sharing,
    link::LinkMatch,
};

use hdk::holochain_json_api::{
    json::{ JsonString, default_to_json},
    error::JsonError
};

use hdk::holochain_persistence_api::{
    cas::content::{Address, AddressableContent}
};

use hdk_proc_macros::zome;

use serde::Serialize;
use std::fmt::Debug;

// see https://developer.holochain.org/api/latest/hdk/ for info on using the hdk library


// An edge. This is an arrow on the SoA Tree which directionally links
// two goals.
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Edge {
    parent_address: Address,
    child_address: Address,
}

// A Goal Card. This is a card on the SoA Tree which can be small or non-small, complete or
// incomplete, certain or uncertain, and contains text content.
// user hash and unix timestamp are included to prevent hash collisions.
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Goal {
    content: String,
    user_hash: Address,
    unix_timestamp: u128,
    complete: bool,
    certain: bool,
    small: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetResponse<T> {
    pub entry: T,
    pub address: Address
}

impl<T: Into<JsonString> + Debug + Serialize> From<GetResponse<T>> for JsonString {
    fn from(u: GetResponse<T>) -> JsonString {
        default_to_json(u)
    }
}

#[zome]
mod my_zome {

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
        let _goals_anchor_address = hdk::commit_entry(&goals_anchor_entry)?;
        let _edges_anchor_address = hdk::commit_entry(&edges_anchor_entry)?;
        Ok(())
    }

    #[validate_agent]
    pub fn validate_agent(validation_data: EntryValidationData<AgentId>) {
        Ok(())
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
                )
            ]
        )
    }

    #[zome_fn("hc_public")]
    fn create_goal(goal: Goal) -> ZomeApiResult<GetResponse<Goal>> {
        let app_entry = Entry::App("goal".into(), goal.clone().into());
        let _ = hdk::commit_entry(&app_entry)?;

        // link each new goal to the anchor
        let anchor_address = Entry::App(
            "anchor".into(), // app entry type
            "goals".into() // app entry value
        ).address();

        hdk::link_entries(&anchor_address, &app_entry.address(),  "anchor->goal", "")?;

        // format the response as a GetResponse
        Ok(GetResponse{entry: goal, address: app_entry.address()})
    }

    #[zome_fn("hc_public")]
    fn create_edge(edge: Edge) -> ZomeApiResult<Edge> {

        let app_entry = Entry::App("edge".into(), edge.clone().into());
        let _ = hdk::commit_entry(&app_entry)?;

        // link each new goal to the anchor
        let anchor_address = Entry::App(
            "anchor".into(), // app entry type
            "edges".into() // app entry value
        ).address();

        hdk::link_entries(&anchor_address, &app_entry.address(),  "anchor->edge", "")?;

        Ok(edge)
    }

    #[zome_fn("hc_public")]
    fn fetch_goals() -> ZomeApiResult<Vec<GetResponse<Goal>>> {
        // set up the anchor entry and compute its hash
        let anchor_address = Entry::App(
            "anchor".into(), // app entry type
            "goals".into(),
        ).address();

        Ok(
            // return all the Goal objects from the entries linked to the edge anchor (drop entries with wrong type)
            hdk::utils::get_links_and_load_type(
                &anchor_address,
                LinkMatch::Exactly("anchor->goal"), // the link type to match
                LinkMatch::Any,
            )?
            // scoop all these entries up into an array and return it
            .into_iter().map(|goal: Goal| {
                // re-create the goal entry to find its address
                let address = Entry::App(
                    "goal".into(),
                    goal.clone().into(),
                ).address();
                // return a response structs with the goal and its address
                GetResponse{entry: goal, address}
            }).collect()
        )
    }

    #[zome_fn("hc_public")]
    fn fetch_edges() -> ZomeApiResult<Vec<Edge>> {
        // set up the anchor entry and compute its hash
        let anchor_address = Entry::App(
            "anchor".into(), // app entry type
            "edges".into(),
        ).address();

        Ok(
            // return all the Edge objects from the entries linked to the edge anchor (drop entries with wrong type)
            hdk::utils::get_links_and_load_type(
                &anchor_address,
                LinkMatch::Exactly("anchor->edge"), // the link type to match
                LinkMatch::Any,
            )?
        )
    }

    #[zome_fn("hc_public")]
    fn archive_goal(address: Address) -> ZomeApiResult<Address> {
        // add the removeEntry
        hdk::remove_entry(&address)?;
        Ok(address)
    }

}
