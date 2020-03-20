extern crate hdk;
extern crate hdk_proc_macros;
extern crate holochain_json_derive;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
use hdk::entry_definition::ValidatingEntryType;

use hdk::holochain_core_types::{dna::entry_types::Sharing, entry::Entry};

pub fn init() -> Result<(), String> {
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
    let goal_members_anchor_entry = Entry::App(
        "anchor".into(), // app entry type
        // app entry value. We'll use the value to specify what this anchor is for
        "goal_members".into(),
    );
    let goal_vote_anchor_entry = Entry::App(
        "anchor".into(), // app entry type
        // app entry value. We'll use the value to specify what this anchor is for
        "goal_votes".into(),
    );
    let goal_comment_anchor_entry = Entry::App(
        "anchor".into(), // app entry type
        // app entry value. We'll use the value to specify what this anchor is for
        "goal_comments".into(),
    );
    let members_anchor_entry = Entry::App(
        "anchor".into(), // app entry type
        // app entry value. We'll use the value to specify what this anchor is for
        "members".into(),
    );
    let projectmeta_anchor_entry = Entry::App(
        "anchor".into(), // app entry type
        // app entry value. We'll use the value to specify what this anchor is for
        "projectmeta".into(),
    );
    hdk::commit_entry(&goal_comment_anchor_entry)?;
    hdk::commit_entry(&goal_vote_anchor_entry)?;
    hdk::commit_entry(&goal_members_anchor_entry)?;
    hdk::commit_entry(&goals_anchor_entry)?;
    hdk::commit_entry(&edges_anchor_entry)?;
    hdk::commit_entry(&members_anchor_entry)?;
    hdk::commit_entry(&projectmeta_anchor_entry)?;
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
                "projectmeta",
                link_type: "anchor->projectmeta",
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },
                validation: | _validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
            to!(
                "member",
                link_type: "anchor->member",
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },
                validation: | _validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
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
            ),
            to!(
                "goal_member",
                link_type: "anchor->goal_member",
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },
                validation: | _validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
            to!(
                "goal_vote",
                link_type: "anchor->goal_vote",
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },
                validation: | _validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            ),
            to!(
                "goal_comment",
                link_type: "anchor->goal_comment",
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
