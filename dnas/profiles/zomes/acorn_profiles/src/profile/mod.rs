use hdk3::prelude::*;

pub const AGENTS_PATH: &str = "agents";

#[derive(Serialize, Deserialize, SerializedBytes)]
pub struct ProfileResponse {
    pub entry: Profile,
    pub address: HeaderHash,
}

#[derive(SerializedBytes, Debug, Clone, PartialEq)]
pub enum Status {
    Online,
    Away,
    Offline,
}
impl From<String> for Status {
    fn from(s: String) -> Self {
        match s.as_str() {
            "Online" => Self::Online,
            "Away" => Self::Away,
            // hack, should be explicit about Offline
            _ => Self::Offline,
        }
    }
}
// for some reason
// default serialization was giving (in json), e.g.
/*
{
  Online: null
}
we just want it Status::Online to serialize to "Online",
so I guess we have to write our own?
*/
impl Serialize for Status {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(match *self {
            Status::Online => "Online",
            Status::Away => "Away",
            Status::Offline => "Offline",
        })
    }
}
impl<'de> Deserialize<'de> for Status {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Online" => Ok(Status::Online),
            "Away" => Ok(Status::Away),
            // hack, should be "Offline"
            _ => Ok(Status::Offline),
        }
    }
}

#[hdk_entry(id = "profile", required_validations = 5)]
#[derive(Clone)]
pub struct Profile {
    first_name: String,
    last_name: String,
    handle: String,
    status: Status,
    avatar_url: String,
    pub address: String,
}

pub fn create_whoami(profile: Profile) -> ExternResult<ProfileResponse> {
    // // send update to peers
    // // notify_new_agent(profile.clone())?;

    // commit this new profile
    let header_hash = commit_entry!(profile.clone())?;

    let entry_hash = entry_hash!(profile.clone())?;

    // list me so anyone can see my profile
    let agents_path_address = Path::from(AGENTS_PATH).hash()?;
    link_entries!(agents_path_address, entry_hash.clone())?;

    // list me so I can specifically and quickly look up my profile
    let agent_pubkey = agent_info!()?.agent_pubkey;
    let agent_entry_hash = EntryHash::from(agent_pubkey);
    link_entries!(agent_entry_hash, entry_hash)?;

    Ok(ProfileResponse {
        entry: profile,
        address: header_hash,
    })
}

#[derive(Serialize, Deserialize, SerializedBytes)]
pub struct UpdateWhoAmIInput {
    profile: Profile,
    address: HeaderHash,
}
pub fn update_whoami(update_who_am_i: UpdateWhoAmIInput) -> ExternResult<ProfileResponse> {
    update_entry!(
        update_who_am_i.address.clone(),
        update_who_am_i.profile.clone()
    )?;
    // // send update to peers
    // // notify_new_agent(profile.clone())?;

    Ok(ProfileResponse {
        entry: update_who_am_i.profile,
        address: update_who_am_i.address,
    })
}

#[derive(Serialize, Deserialize, SerializedBytes)]
pub struct UpdateStatusInput {
    status: String,
}
pub fn update_status(update_status_input: UpdateStatusInput) -> ExternResult<ProfileResponse> {
    match whoami()?.0 {
        Some(ProfileResponse { entry, address }) => update_whoami(UpdateWhoAmIInput {
            profile: Profile {
                first_name: entry.first_name,
                last_name: entry.last_name,
                handle: entry.handle,
                avatar_url: entry.avatar_url,
                address: entry.address,
                status: Status::from(update_status_input.status),
                // ..entry
            },
            address,
        }),
        _ => Err(WasmError::Zome(
            "Could not retrieve the agent profile".into(),
        )),
    }
}

#[derive(Serialize, Deserialize, SerializedBytes)]
pub struct WhoAmIOutput(Option<ProfileResponse>);

fn get_latest_for_entry(entry_address: EntryHash) -> ExternResult<Option<(Entry, HeaderHash)>> {
    // First, make sure we DO have the latest entry address
    let maybe_latest_entry_address = match get_details!(entry_address.clone())? {
        Some(Details::Entry(details)) => match details.updates.len() {
            0 => Some(entry_address),
            // at least 1 update, get the LAST one
            // I DUNNO, is the LAST one the most recent one?
            // How are they sorted?
            _ => {
              let mut sortlist = details.updates.to_vec();
              // unix timestamp should work for sorting
              // ascending or descending?
              sortlist.sort_by_key(|header| header.timestamp.0);
              // debug!(sortlist);
              let _ = debug!(sortlist.len());
              Some(sortlist.first().unwrap().entry_hash.clone())
            },
        },
        _ => None,
    };
    
    // Second, go and get that entry, and return it and its header_address
    match maybe_latest_entry_address {
      Some(latest_entry_address) => match get!(latest_entry_address)? {
        Some(element) => match element.entry().as_option() {
          Some(entry) => Ok(Some((entry.to_owned(), element.header_address().to_owned()))),
          None => Ok(None)
        },
        None => Ok(None)
      },
      None => Ok(None)
    }
}

pub fn whoami() -> ExternResult<WhoAmIOutput> {
    let agent_pubkey = agent_info!()?.agent_pubkey;
    let agent_entry_hash = EntryHash::from(agent_pubkey);

    let all_profiles = get_links!(agent_entry_hash)?.into_inner();
    let maybe_profile_link = all_profiles.last();
    // // do it this way so that we always keep the original profile entry address
    // // from the UI perspective
    match maybe_profile_link {
        Some(profile_link) => match get_latest_for_entry(profile_link.target.clone())? {
            Some((entry, header_address)) => match entry {
                Entry::App(sb) => {
                    let profile = Profile::try_from(sb.to_owned())?;
                    Ok(WhoAmIOutput(Some(ProfileResponse {
                        entry: profile,
                        address: header_address,
                    })))
                }
                _ => Ok(WhoAmIOutput(None)),
            },
            None => Ok(WhoAmIOutput(None)),
        },
        None => Ok(WhoAmIOutput(None)),
    }
}

#[derive(Serialize, Deserialize, SerializedBytes)]
pub struct AgentsOutput(Vec<Profile>);

struct GetError(());

pub fn fetch_agents() -> ExternResult<AgentsOutput> {
    let agents_path_address = Path::from(AGENTS_PATH).hash()?;
    let links = get_links!(agents_path_address)?;
    let agents = links
        .into_inner()
        .into_iter()
        .map(|link| match get!(link.target) {
            Ok(Some(element)) => match element.entry().to_app_option::<Profile>() {
                Ok(Some(profile)) => Ok(profile),
                _ => Err(GetError(())),
            },
            _ => Err(GetError(())),
        })
        .filter_map(Result::ok)
        .collect();
    Ok(
        // return all the Profile objects from the entries linked to the profiles anchor (drop entries with wrong type)
        AgentsOutput(agents),
    )
}

// pub fn profile_def() -> ValidatingEntryType {
//     entry!(
//         name: "profile",
//         description: "this is an entry representing some profile info for an agent",
//         sharing: Sharing::Public,
//         validation_package: || {
//             hdk::ValidationPackageDefinition::Entry
//         },
//         validation: | validation_data: hdk::EntryValidationData<Profile>| {
//             match validation_data{
//                 hdk::EntryValidationData::Create{entry,validation_data}=>{
//                     let agent_address = &validation_data.sources()[0];
//                     if entry.address!=agent_address.to_string() {
//                         Err("only the same agent as the profile is about can create their profile".into())
//                     }else {Ok(())}
//                 },
//                 hdk::EntryValidationData::Modify{
//                     new_entry,
//                     old_entry,validation_data,..}=>{
//                     let agent_address = &validation_data.sources()[0];
//                     if new_entry.address!=agent_address.to_string()&& old_entry.address!=agent_address.to_string(){
//                         Err("only the same agent as the profile is about can modify their profile".into())
//                     }else {Ok(())}
//                 },
//                 hdk::EntryValidationData::Delete{old_entry,validation_data,..}=>{
//                     let agent_address = &validation_data.sources()[0];
//                     if old_entry.address!=agent_address.to_string() {
//                         Err("only the same agent as the profile is about can delete their profile".into())
//                     }else {Ok(())}
//                 }
//             }
//         },
//         links: [
//             from!(
//                 "%agent_id",
//                 link_type: "agent->profile",
//                 validation_package: || {
//                     hdk::ValidationPackageDefinition::Entry
//                 },
//                validation: |link_validation_data: hdk::LinkValidationData| {
//                     let validation_data =
//                         match link_validation_data {
//                             hdk::LinkValidationData::LinkAdd {
//                                 validation_data,..
//                             } => validation_data,
//                             hdk::LinkValidationData::LinkRemove {
//                                 validation_data,..
//                             } =>validation_data,
//                         };
//                     let agent_address=&validation_data.sources()[0];
//                     if let Some(vector)= validation_data.package.source_chain_entries{
//                         if let App (_,entry)=&vector[0]{
//                         if let Ok(profile)=serde_json::from_str::<Profile>(&Into::<String>::into(entry)) {
//                             if profile.address==agent_address.to_string(){
//                             Ok(())

//                             }else {
//                         Err("Cannot edit other people's Profile1".to_string())}
//                         }else {
//                         Err("Cannot edit other people's Profile2".to_string())}
//                     }
//                     else{
//                         Err("Cannot edit other people's Profile3".to_string())
//                     }

//                     } else{
//                         Ok(())
//                     }
//                     }
//             )
//         ]
//     )
// }

// send the direct messages that will result in
// signals being emitted to the UI
// fn notify_all(message: DirectMessage) -> ExternResult<()> {
//     fetch_agents()?.iter().for_each(|profile| {
//         // useful for development purposes
//         // uncomment to send signals to oneself
//         // if profile.address == AGENT_ADDRESS.to_string() {
//         //     signal_ui(&message);
//         // }

//         if profile.address != AGENT_ADDRESS.to_string() {
//             hdk::debug(format!(
//                 "Send a message to: {:?}",
//                 &profile.address.to_string()
//             ))
//             .ok();
//             hdk::send(
//                 Address::from(profile.address.clone()),
//                 JsonString::from(message.clone()).into(),
//                 1.into(),
//             )
//             .ok();
//         }
//     });
//     Ok(())
// }

// fn notify_new_agent(profile: Profile) -> ExternResult<()> {
//     let message = DirectMessage::NewAgentNotification(NewAgentSignalPayload { agent: profile });
//     notify_all(message)
// }
