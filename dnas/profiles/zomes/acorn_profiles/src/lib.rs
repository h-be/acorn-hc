use dna_help::*;
use hdk3::prelude::*;

mod profile;

use profile::{Profile, AGENTS_PATH};

#[hdk_extern]
fn init(_: ()) -> ExternResult<InitCallbackResult> {
    Path::from(AGENTS_PATH).ensure()?;
    Ok(InitCallbackResult::Pass)
}

entry_defs![Path::entry_def(), Profile::entry_def()];

#[derive(Serialize, Deserialize, SerializedBytes, Clone)]
pub struct Test {
    address: WrappedAgentPubKey,
}

#[hdk_extern]
fn test(_: ()) -> ExternResult<Test> {
    let address = agent_info!()?.agent_latest_pubkey;
    Ok(Test {
        address: WrappedAgentPubKey(address),
    })
}

#[hdk_extern]
fn test2(_: Test) -> ExternResult<()> {
    Ok(())
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
