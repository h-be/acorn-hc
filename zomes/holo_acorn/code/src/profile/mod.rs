extern crate hdk;
extern crate hdk_proc_macros;
extern crate serde;

extern crate serde_derive;
extern crate serde_json;

extern crate holochain_json_derive;
use hdk::holochain_core_types::{
    // agent::AgentId, dna::entry_types::Sharing, entry::Entry, link::LinkMatch,
    dna::entry_types::Sharing,
    entry::Entry,
    link::LinkMatch,
};
use hdk::prelude::Entry::App;
use hdk::{
    entry_definition::ValidatingEntryType,
    error::{ZomeApiError, ZomeApiResult},
    // AGENT_ADDRESS, AGENT_ID_STR,
    AGENT_ADDRESS,
};

use hdk::holochain_json_api::{
    error::JsonError,
    json::{default_to_json, JsonString},
};

use hdk::holochain_persistence_api::cas::content::{Address, AddressableContent};

use serde::Serialize;
//use std::convert::{TryFrom};
// use std::convert::{TryFrom, TryInto};
use std::fmt::Debug;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GetResponse<T> {
    pub entry: T,
    pub address: Address,
}

impl<T: Into<JsonString> + Debug + Serialize> From<GetResponse<T>> for JsonString {
    fn from(u: GetResponse<T>) -> JsonString {
        default_to_json(u)
    }
}
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub enum Status {
    Online,
    Away,
    Offline,
}
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Profile {
    first_name: String,
    last_name: String,
    handle: String,
    status: Status,
    avatar_url: String,
    address: String,
}
pub fn profile_def() -> ValidatingEntryType {
    entry!(
        name: "profile",
        description: "this is an entry representing some profile info for an agent",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | validation_data: hdk::EntryValidationData<Profile>| {
            match validation_data{
                hdk::EntryValidationData::Create{entry,validation_data}=>{
                    let agent_address = &validation_data.sources()[0];
                    if entry.address!=agent_address.to_string() {
                        Err("only the same agent as the profile is about can create their profile".into())
                    }else {Ok(())}
                },
                hdk::EntryValidationData::Modify{
                    new_entry,
                    old_entry,validation_data,..}=>{
                    let agent_address = &validation_data.sources()[0];
                    if new_entry.address!=agent_address.to_string()&& old_entry.address!=agent_address.to_string(){
                        Err("only the same agent as the profile is about can modify their profile".into())
                    }else {Ok(())}
                },
                hdk::EntryValidationData::Delete{old_entry,validation_data,..}=>{
                    let agent_address = &validation_data.sources()[0];
                    if old_entry.address!=agent_address.to_string() {
                        Err("only the same agent as the profile is about can delete their profile".into())
                    }else {Ok(())}
                }
            }
        },
        links: [
            from!(
                "%agent_id",
                link_type: "agent->profile",
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },
               validation: |link_validation_data: hdk::LinkValidationData| {
                    let validation_data =
                        match link_validation_data {
                            hdk::LinkValidationData::LinkAdd {
                                validation_data,..
                            } => validation_data,
                            hdk::LinkValidationData::LinkRemove {
                                validation_data,..
                            } =>validation_data,
                        };
                    let agent_address=&validation_data.sources()[0];
                    if let Some(vector)= validation_data.package.source_chain_entries{
                        if let App (_,entry)=&vector[0]{
                        if let Ok(profile)=serde_json::from_str::<Profile>(&Into::<String>::into(entry)) {
                            if profile.address==agent_address.to_string(){
                            Ok(())

                            }else {
                        Err("Cannot edit other people's Profile1".to_string())}
                        }else {
                        Err("Cannot edit other people's Profile2".to_string())}
                    }
                    else{
                        Err("Cannot edit other people's Profile3".to_string())
                    }

                    } else{
                        Ok(())
                    }
                    }
            )
        ]
    )
}
pub fn create_whoami(profile: Profile) -> ZomeApiResult<GetResponse<Profile>> {
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
    hdk::link_entries(&AGENT_ADDRESS, &profile_address, "agent->profile", "")?;
    Ok(GetResponse {
        entry: profile,
        address: profile_address,
    })
}
pub fn update_whoami(profile: Profile, address: Address) -> ZomeApiResult<GetResponse<Profile>> {
    let profile_entry = Entry::App("profile".into(), profile.clone().into());
    hdk::update_entry(profile_entry, &address)?;
    Ok(GetResponse {
        entry: profile,
        address: address,
    })
}
pub fn update_status(status: Status) -> ZomeApiResult<GetResponse<Profile>> {
    if let Some(GetResponse { entry, address }) = whoami()? {
        update_whoami(
            Profile {
                first_name: entry.first_name,
                last_name: entry.last_name,
                handle: entry.handle,
                status: status,

                avatar_url: entry.avatar_url,
                address: entry.address,
            },
            address,
        )
    } else {
        Err(ZomeApiError::Internal("error".into()))
    }
}
pub fn whoami() -> ZomeApiResult<Option<GetResponse<Profile>>> {
    match hdk::utils::get_links_and_load_type::<Profile>(
        &AGENT_ADDRESS,
        LinkMatch::Exactly("agent->profile"), // the link type to match
        LinkMatch::Any,
    )?
    .first()
    {
        Some(my_profile) => {
            let app_entry = Entry::App("profile".into(), my_profile.into());
            Ok(Some(GetResponse {
                entry: my_profile.clone(),
                address: app_entry.address(),
            }))
        }
        None => Ok(None),
    }
}
pub fn fetch_agents() -> ZomeApiResult<Vec<Profile>> {
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
        )?,
    )
}
