use super::goal::Goal;
use dna_help::{fetch_links, get_latest_for_entry, EntryAndHash};
use hdk3::prelude::*;

pub const ENTRY_POINT_PATH: &str = "entry_point";

// The "Entry" in EntryPoint is not a reference to Holochain "Entries"
// it is rather the concept of an Entrance, as in a doorway, to the tree
#[hdk_entry(id = "entry_point")]
#[derive(Debug, Clone, PartialEq)]
pub struct EntryPoint {
    color: String,
    creator_address: AgentPubKey,
    created_at: f64,
    goal_address: EntryHash,
}

#[derive(Serialize, Deserialize, Debug, SerializedBytes, Clone, PartialEq)]
pub struct EntryPointResponse {
    entry_point: EntryPoint,
    goal: Goal,
    entry_point_address: HeaderHash,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, SerializedBytes)]
struct WireEntry {
    pub entry: EntryPoint,
    pub address: HeaderHash,
}

impl From<EntryAndHash<EntryPoint>> for WireEntry {
    fn from(entry_and_hash: EntryAndHash<EntryPoint>) -> Self {
        WireEntry {
            entry: entry_and_hash.0,
            address: entry_and_hash.1,
        }
    }
}

#[derive(Serialize, Deserialize, SerializedBytes)]
pub struct VecEntryPointResponse (Vec<EntryPointResponse>);

// CREATE
#[hdk_extern]
pub fn create_entry_point(entry: EntryPoint) -> ExternResult<EntryPointResponse> {
    // if the goal doesn't exist, this will return an error for this function
    // which is right
    match get_latest_for_entry::<Goal>(entry.goal_address.clone())? {
        Some((goal, _)) => {
            let address = create_entry!(entry.clone())?;
            let entry_hash = hash_entry!(entry.clone())?;
            let path_hash = Path::from(ENTRY_POINT_PATH).hash()?;
            create_link!(path_hash, entry_hash)?;
            let wire_entry = EntryPointResponse {
                entry_point: entry,
                // can safely use unwrap here because
                // we checked against the None case
                goal,
                entry_point_address: address,
            };
            // notify_entry_point(entry_point_response.clone())?;
            Ok(wire_entry)
        }
        None => Err(HdkError::Wasm(WasmError::Zome(
            "associated goal doesn't exists".into(),
        ))),
    }
}

// READ
#[hdk_extern]
pub fn fetch_entry_points(_: ()) -> ExternResult<VecEntryPointResponse> {
    let path_hash = Path::from(ENTRY_POINT_PATH).hash()?;
    let entries = fetch_links::<EntryPoint, WireEntry>(path_hash)?
        .into_iter()
        .map(|wire_entry| {
            match get_latest_for_entry::<Goal>(wire_entry.entry.goal_address.clone()) {
                Ok(Some((goal, _))) => Ok(EntryPointResponse {
                    entry_point: wire_entry.entry,
                    goal,
                    entry_point_address: wire_entry.address,
                }),
                Ok(None) => Err(HdkError::Wasm(WasmError::Zome(
                    "the associated goal for this entry point wasn't found".into(),
                ))),
                Err(e) => Err(e),
            }
        })
        // drop errored results, and map to the succesful inner values
        .filter_map(Result::ok)
        .collect();
    Ok(VecEntryPointResponse(entries))
}

// DELETE
#[hdk_extern]
pub fn archive_entry_point(address: HeaderHash) -> ExternResult<HeaderHash> {
    delete_entry!(address.clone())?;
    // notify_entry_point_archived(address.clone())?;
    Ok(address)
}
