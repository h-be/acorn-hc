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
    AGENT_ADDRESS,
};
use std::convert::TryInto;

use hdk_proc_macros::zome;

mod profile;
use crate::profile::{GetResponse, Profile, Status};

mod anchor;

//The GetResponse struct allows our zome functions to return an entry along with its
//address so that Redux can know the address of goals and edges

// these types will come straight through signals to the UI,
// so they will actually be referenced there. Be mindful of this
pub const NEW_AGENT_SIGNAL_TYPE: &str = "new_agent";

#[derive(Clone, Debug, Serialize, Deserialize, DefaultJson, PartialEq)]
#[serde(rename_all = "camelCase")]
struct NewAgentSignalPayload {
    agent: Profile,
}

/// Fully typed definition of the types of direct messages.
/// All of which exist for the purposes of UI signals
/// at this time.
#[derive(Clone, Serialize, Deserialize, Debug, DefaultJson, PartialEq)]
pub(crate) enum DirectMessage {
    NewAgentNotification(NewAgentSignalPayload),
}

// send a signal to the UI
pub(crate) fn signal_ui(message: &DirectMessage) {
    match message {
        // Agents
        DirectMessage::NewAgentNotification(signal_payload) => {
            hdk::emit_signal(NEW_AGENT_SIGNAL_TYPE, signal_payload).ok();
        }
    };
}

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
    fn profile_def() -> ValidatingEntryType {
        profile::profile_def()
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
    fn update_status(status: Status) -> ZomeApiResult<GetResponse<Profile>> {
        profile::update_status(status)
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
    fn fetch_agents() -> ZomeApiResult<Vec<Profile>> {
        profile::fetch_agents()
    }
}
