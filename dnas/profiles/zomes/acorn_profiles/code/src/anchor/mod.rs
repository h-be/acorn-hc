extern crate hdk;
extern crate hdk_proc_macros;
extern crate holochain_json_derive;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
use hdk::entry_definition::ValidatingEntryType;

use hdk::holochain_core_types::{dna::entry_types::Sharing, entry::Entry};

pub fn init() -> Result<(), String> {
    let agents_anchor_entry = Entry::App(
        "anchor".into(), // app entry type
        // app entry value. We'll use the value to specify what this anchor is for
        "agents".into(),
    );
    hdk::commit_entry(&agents_anchor_entry)?;
    Ok(())
}
pub fn anchor_def() -> ValidatingEntryType {
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
                "profile",
                link_type: "anchor->profiles",
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
