use dna_help::crud;
use hdk3::prelude::*;

#[hdk_entry(id = "project_meta")]
#[derive(Debug, Clone, PartialEq)]
pub struct ProjectMeta {
    creator_address: AgentPubKey,
    created_at: f64,
    name: String,
    image: Option<String>,
    passphrase: String,
}

crud!(ProjectMeta, project_meta, "project_meta");

// READ
#[hdk_extern]
pub fn fetch_project_meta(_: ()) -> ExternResult<ProjectMetaWireEntry> {
    match inner_fetch_project_metas()?.0.first() {
        Some(wire_entry) => Ok(wire_entry.clone()),
        None => Err(HdkError::Wasm(WasmError::Zome(
            "no project meta exists".into(),
        ))),
    }
}
