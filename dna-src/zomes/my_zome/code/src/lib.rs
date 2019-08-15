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

use std::convert::TryFrom;
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
    json::JsonString,
    error::JsonError
};

use hdk::holochain_persistence_api::{
    cas::content::Address
};

use hdk_proc_macros::zome;

// see https://developer.holochain.org/api/latest/hdk/ for info on using the hdk library

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]

// A Goal Card. This is a card on the SoA Tree which can be small or non-small, complete or
// incomplete, certain or uncertain, and contains text content.
// user hash and unix timestamp are included to prevent hash collisions.

pub struct Goal {
    content: String,
    user_hash: Address,
    unix_timestamp: u32,
    complete: bool,
    certain: bool,
    small: bool,
}

#[zome]
mod my_zome {

    #[init]
    pub fn init() {
        // create anchor entry
        let anchor_entry = Entry::App(
            "anchor".into(), // app entry type
            // app entry value. We'll use the value to specify what this anchor is for
            "goals".into(),
        );
        let _anchor_address = hdk::commit_entry(&anchor_entry)?;
        Ok(())
    }

    #[validate_agent]
    pub fn validate_agent(validation_data: EntryValidationData<AgentId>) {
        Ok(())
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
                )
            ]
        )
    }

    #[zome_fn("hc_public")]
    fn create_goal(goal: Goal) -> ZomeApiResult<Goal> {
        let app_entry = Entry::App("goal".into(), goal.clone().into());
        let _ = hdk::commit_entry(&app_entry)?;

        // link each new goal to the anchor
        let anchor_entry = Entry::App(
            "anchor".into(), // app entry type
            "goals".into() // app entry value
        );
        let anchor_address = hdk::entry_address(&anchor_entry).unwrap();

        hdk::link_entries(&anchor_address, &hdk::entry_address(&app_entry).unwrap(),  "anchor->goal", "")?;
        Ok(goal)
    }

    #[zome_fn("hc_public")]
    fn fetch_goals() -> ZomeApiResult<Vec<Goal>> {
        let anchor_address = hdk::entry_address(&Entry::App("anchor".into(), "goals".into())).unwrap();
        let links = hdk::get_links(&anchor_address, LinkMatch::Exactly("anchor->goal"), LinkMatch::Any)?;

        match links.addresses().into_iter().next() {
            Some(first_goal) => {
                let mut goal_addresses = vec![first_goal];
                let mut there_are_more = true;
                // keep adding addresses to the list as long as there are more links
                while there_are_more {
                    there_are_more = match hdk::get_links(goal_addresses.last().unwrap(), LinkMatch::Exactly("anchor->goal"), LinkMatch::Any)?.addresses().into_iter().next() {
                        Some(addr) => {
                            goal_addresses.push(addr.clone());
                            true
                        },
                        None => {
                            false
                        },
                    }
                }
                let goals: Vec<Goal> = goal_addresses.iter().map(|addr| {
                    let goal_entry = hdk::get_entry(addr).unwrap().unwrap();
                    if let Entry::App(_, goal_struct) = goal_entry {
                        Goal::try_from(goal_struct).expect("Entry at address is type other than Goal")
                    } else {
                        panic!("Not an app entry!")
                    }
                }).collect();
                Ok(goals)
            },
            None => {
                Ok(Vec::new())
            },
        }
    }

}
