use dna_help::create_receive_signal_cap_grant;
use hdk3::prelude::*;

mod profile;

use profile::{Profile, AGENTS_PATH};

#[hdk_extern]
fn init(_: ()) -> ExternResult<InitCallbackResult> {
    Path::from(AGENTS_PATH).ensure()?;
    create_receive_signal_cap_grant()?;
    Ok(InitCallbackResult::Pass)
}

entry_defs![Path::entry_def(), Profile::entry_def()];
