use dna_help::{EntryAndHash, WrappedAgentPubKey, fetch_links};
use hdk3::prelude::*;

pub const MEMBER_PATH: &str = "member";

// This is a reference to the agent address for any users who have joined this DHT
#[hdk_entry(id = "member")]
#[derive(Debug, Clone, PartialEq)]
pub struct Member {
    pub address: WrappedAgentPubKey,
}

impl From<EntryAndHash<Member>> for Member {
    fn from(entry_and_hash: EntryAndHash<Member>) -> Self {
        entry_and_hash.0
    }
}

#[derive(Serialize, Deserialize, SerializedBytes)]
pub struct VecMember(Vec<Member>);

#[hdk_extern]
pub fn fetch_members(_: ()) -> ExternResult<VecMember> {
    let path_hash = Path::from(MEMBER_PATH).hash()?;
    let entries = fetch_links::<Member, Member>(path_hash)?;
    Ok(VecMember(entries))
}
