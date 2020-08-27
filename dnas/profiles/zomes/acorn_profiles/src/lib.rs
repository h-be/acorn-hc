use hdk3::prelude::*;

mod profile;
use crate::profile::{
    AgentsOutput, Profile, ProfileResponse, UpdateWhoAmIInput,
    WhoAmIOutput, AGENTS_PATH,
};

#[hdk_extern]
fn init(_: ()) -> ExternResult<InitCallbackResult> {
    Path::from(AGENTS_PATH).ensure()?;
    Ok(InitCallbackResult::Pass)
}

entry_defs![Path::entry_def(), Profile::entry_def()];

#[hdk_extern]
fn create_whoami(profile: Profile) -> ExternResult<ProfileResponse> {
    profile::create_whoami(profile)
}

#[hdk_extern]
fn update_whoami(who_am_i_input: UpdateWhoAmIInput) -> ExternResult<ProfileResponse> {
    profile::update_whoami(who_am_i_input)
}

#[hdk_extern]
fn whoami(_: ()) -> ExternResult<WhoAmIOutput> {
    profile::whoami()
}

#[derive(Serialize, Deserialize, SerializedBytes)]
struct AgentAddressOutput(String);

// fn fetch_agent_address(_: ()) -> ExternResult<AgentPubKey> {
#[hdk_extern]
fn fetch_agent_address(_: ()) -> ExternResult<AgentAddressOutput> {
    let agent_info = agent_info!()?;
    Ok(AgentAddressOutput(agent_info.agent_pubkey.to_string()))
}

#[hdk_extern]
fn fetch_agents(_: ()) -> ExternResult<AgentsOutput> {
    profile::fetch_agents()
}

// The GetResponse struct allows our zome functions to return an entry along with its
// address so that Redux can know the address of goals and edges

// these types will come straight through signals to the UI,
// so they will actually be referenced there. Be mindful of this
// pub const NEW_AGENT_SIGNAL_TYPE: &str = "new_agent";

// #[derive(Clone, Debug, Serialize, Deserialize, DefaultJson, PartialEq)]
// #[serde(rename_all = "camelCase")]
// struct NewAgentSignalPayload {
//     agent: Profile,
// }

// Fully typed definition of the types of direct messages.
// All of which exist for the purposes of UI signals
// at this time.
// #[derive(Clone, Serialize, Deserialize, Debug, DefaultJson, PartialEq)]
// pub(crate) enum DirectMessage {
//     NewAgentNotification(NewAgentSignalPayload),
// }

// send a signal to the UI
// pub(crate) fn signal_ui(message: &DirectMessage) {
//     match message {
//         // Agents
//         DirectMessage::NewAgentNotification(signal_payload) => {
//             hdk::emit_signal(NEW_AGENT_SIGNAL_TYPE, signal_payload).ok();
//         }
//     };
// }
// #[receive]
// pub fn receive(from: Address, msg_json: JsonString) -> String {
//     hdk::debug(format!("New direct message from: {:?}", from)).ok();
//     let maybe_message: Result<DirectMessage, _> = JsonString::from_json(&msg_json).try_into();
//     match maybe_message {
//         Err(err) => format!("Err({})", err),
//         Ok(message) => {
//             signal_ui(&message);
//             String::from("Ok")
//         }
//     }
// }
