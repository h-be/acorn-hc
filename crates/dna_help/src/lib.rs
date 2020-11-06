use hdk3::prelude::*;
pub use paste;

pub type EntryAndHash<T> = (T, HeaderHash, EntryHash);
pub type OptionEntryAndHash<T> = Option<EntryAndHash<T>>;

#[derive(Debug, Serialize, Deserialize, SerializedBytes, Clone, PartialEq)]
pub struct UIStringHash(String);

#[derive(Debug, Serialize, Deserialize, SerializedBytes, Clone, PartialEq)]
#[serde(try_from = "UIStringHash")]
#[serde(into = "UIStringHash")]
pub struct WrappedAgentPubKey(pub AgentPubKey);

#[derive(Debug, Serialize, Deserialize, SerializedBytes, Clone, PartialEq)]
#[serde(try_from = "UIStringHash")]
#[serde(into = "UIStringHash")]
pub struct WrappedHeaderHash(pub HeaderHash);

#[derive(Debug, Serialize, Deserialize, SerializedBytes, Clone, PartialEq)]
#[serde(try_from = "UIStringHash")]
#[serde(into = "UIStringHash")]
pub struct WrappedEntryHash(pub EntryHash);

impl TryFrom<UIStringHash> for WrappedAgentPubKey {
    type Error = String;
    fn try_from(ui_string_hash: UIStringHash) -> Result<Self, Self::Error> {
        match AgentPubKey::try_from(ui_string_hash.0) {
            Ok(address) => Ok(Self(address)),
            Err(e) => Err(format!("{:?}", e)),
        }
    }
}
impl From<WrappedAgentPubKey> for UIStringHash {
    fn from(wrapped_agent_pub_key: WrappedAgentPubKey) -> Self {
        Self(wrapped_agent_pub_key.0.to_string())
    }
}

impl TryFrom<UIStringHash> for WrappedHeaderHash {
    type Error = String;
    fn try_from(ui_string_hash: UIStringHash) -> Result<Self, Self::Error> {
        match HeaderHash::try_from(ui_string_hash.0) {
            Ok(address) => Ok(Self(address)),
            Err(e) => Err(format!("{:?}", e)),
        }
    }
}
impl From<WrappedHeaderHash> for UIStringHash {
    fn from(wrapped_header_hash: WrappedHeaderHash) -> Self {
        Self(wrapped_header_hash.0.to_string())
    }
}

impl TryFrom<UIStringHash> for WrappedEntryHash {
    type Error = String;
    fn try_from(ui_string_hash: UIStringHash) -> Result<Self, Self::Error> {
        match EntryHash::try_from(ui_string_hash.0) {
            Ok(address) => Ok(Self(address)),
            Err(e) => Err(format!("{:?}", e)),
        }
    }
}
impl From<WrappedEntryHash> for UIStringHash {
    fn from(wrapped_entry_hash: WrappedEntryHash) -> Self {
        Self(wrapped_entry_hash.0.to_string())
    }
}

/*
  SIGNALS
*/

// sender
pub fn signal_peers<T: TryInto<SerializedBytes>>(signal: T, peers: Vec<AgentPubKey>) -> ExternResult<()> {
  let zome_info = zome_info!()?;
  match signal.try_into() {
    Ok(sb) => {
      for peer in peers {
        // ignore errors
        let _ = call_remote!(
          peer,
          zome_info.zome_name.clone(),
          zome::FunctionName("receive_signal".into()),
          None,
          sb.clone()
        );
      };
      Ok(())
    },
    Err(_) => Err(
      HdkError::SerializedBytes(
        SerializedBytesError::ToBytes(
          "couldnt convert signal into serializedbytes".to_string()
        )
      )
    )
  }
}

pub fn create_receive_signal_cap_grant() -> ExternResult<()> {
  let mut functions: GrantedFunctions = HashSet::new();
  functions.insert((zome_info!()?.zome_name, "receive_signal".into()));

  create_cap_grant!(CapGrantEntry {
      tag: "".into(),
      // empty access converts to unrestricted
      access: ().into(),
      functions,
  })?;
  Ok(())
}


pub fn get_header_hash(shh: element::SignedHeaderHashed) -> HeaderHash {
    shh.header_hashed().as_hash().to_owned()
}

pub fn get_latest_for_entry<T: TryFrom<SerializedBytes, Error = SerializedBytesError>>(
    entry_hash: EntryHash,
) -> ExternResult<OptionEntryAndHash<T>> {
    // First, make sure we DO have the latest header_hash address
    let maybe_latest_header_hash = match get_details!(entry_hash.clone())? {
        Some(Details::Entry(details)) => match details.entry_dht_status {
            metadata::EntryDhtStatus::Live => match details.updates.len() {
                // pass out the header associated with this entry
                0 => Some(get_header_hash(details.headers.first().unwrap().to_owned())),
                _ => {
                    let mut sortlist = details.updates.to_vec();
                    // unix timestamp should work for sorting
                    sortlist.sort_by_key(|update| update.header().timestamp().0);
                    // sorts in ascending order, so take the last element
                    let last = sortlist.last().unwrap().to_owned();
                    Some(get_header_hash(last))
                }
            },
            metadata::EntryDhtStatus::Dead => None,
            _ => None,
        },
        _ => None,
    };

    // Second, go and get that element, and return it and its header_address
    match maybe_latest_header_hash {
        Some(latest_header_hash) => match get!(latest_header_hash)? {
            Some(element) => match element.entry().to_app_option::<T>()? {
                Some(entry) => Ok(Some((
                    entry,
                    match element.header() {
                        // we DO want to return the header for the original
                        // instead of the updated, in our case
                        Header::Update(update) => update.original_header_address.clone(),
                        Header::Create(_) => element.header_address().clone(),
                        _ => unreachable!("Can't have returned a header for a nonexistent entry"),
                    },
                    element.header().entry_hash().unwrap().to_owned(),
                ))),
                None => Ok(None),
            },
            None => Ok(None),
        },
        None => Ok(None),
    }
}

pub fn fetch_links<
    EntryType: TryFrom<SerializedBytes, Error = SerializedBytesError>,
    WireEntry: From<EntryAndHash<EntryType>>,
>(
    entry_hash: EntryHash,
) -> Result<Vec<WireEntry>, SerializedBytesError> {
    Ok(get_links!(entry_hash)?
        .into_inner()
        .into_iter()
        .map(|link: link::Link| get_latest_for_entry::<EntryType>(link.target.clone()))
        .filter_map(Result::ok)
        .filter_map(|x| x)
        .map(|x| WireEntry::from(x))
        .collect())
}

#[macro_export]
macro_rules! crud {
    (
      $crud_type:ident, $i:ident, $path:expr
    ) => {

        $crate::paste::paste! {
          pub const [<$i:upper _PATH>]: &str = $path;

          #[derive(Serialize, Deserialize, Debug, Clone, PartialEq, SerializedBytes)]
          pub struct [<$crud_type WireEntry>] {
            pub entry: $crud_type,
            pub address: $crate::WrappedHeaderHash,
            pub entry_address: $crate::WrappedEntryHash
          }

          #[derive(Serialize, Deserialize, Debug, Clone, PartialEq, SerializedBytes)]
          pub struct [<$crud_type UpdateInput>] {
            pub entry: $crud_type,
            pub address: $crate::WrappedHeaderHash,
          }

          impl From<$crate::EntryAndHash<$crud_type>> for [<$crud_type WireEntry>] {
            fn from(entry_and_hash: $crate::EntryAndHash<$crud_type>) -> Self {
              [<$crud_type WireEntry>] {
                entry: entry_and_hash.0,
                address: $crate::WrappedHeaderHash(entry_and_hash.1),
                entry_address: $crate::WrappedEntryHash(entry_and_hash.2)
              }
            }
          }

          #[derive(Serialize, Deserialize, Debug, Clone, PartialEq, SerializedBytes)]
          pub struct [<Vec $crud_type WireEntry>](pub Vec<[<$crud_type WireEntry>]>);

          /*
            CREATE
          */
          pub fn [<inner_create_ $i>](entry: $crud_type) -> ExternResult<[<$crud_type WireEntry>]> {
            let address = create_entry!(entry.clone())?;
            let entry_hash = hash_entry!(entry.clone())?;
            let path_hash = Path::from([<$i:upper _PATH>]).hash()?;
            create_link!(path_hash, entry_hash.clone())?;
            let wire_entry = [<$crud_type WireEntry>] {
              entry,
              address: $crate::WrappedHeaderHash(address),
              entry_address: $crate::WrappedEntryHash(entry_hash)
            };
            // notify_goal_comment(wire_entry.clone())?;
            Ok(wire_entry)
          }

          #[hdk_extern]
          pub fn [<create_ $i>](entry: $crud_type) -> ExternResult<[<$crud_type WireEntry>]> {
            [<inner_create_ $i>](entry)
          }

          /*
            READ
          */
          pub fn [<inner_fetch_ $i s>]() -> ExternResult<[<Vec $crud_type WireEntry>]> {
            let path_hash = Path::from([<$i:upper _PATH>]).hash()?;
            let entries = $crate::fetch_links::<$crud_type, [<$crud_type WireEntry>]>(path_hash)?;
            Ok([<Vec $crud_type WireEntry>](entries))
          }

          #[hdk_extern]
          pub fn [<fetch_ $i s>](_: ()) -> ExternResult<[<Vec $crud_type WireEntry>]> {
            [<inner_fetch_ $i s>]()
          }

          /*
            UPDATE
          */
          pub fn [<inner_update_ $i>](update: [<$crud_type UpdateInput>]) -> ExternResult<[<$crud_type WireEntry>]> {
            update_entry!(update.address.0.clone(), update.entry.clone())?;
            let entry_address = hash_entry!(update.entry.clone())?;
            let wire_entry = [<$crud_type WireEntry>] {
                entry: update.entry,
                address: update.address,
                entry_address: $crate::WrappedEntryHash(entry_address)
            };
            // notify_goal_comment(wire_entry.clone())?;
            Ok(wire_entry)
          }

          #[hdk_extern]
          pub fn [<update_ $i>](update: [<$crud_type UpdateInput>]) -> ExternResult<[<$crud_type WireEntry>]> {
            [<inner_update_ $i>](update)
          }

          /*
            DELETE
          */
          pub fn [<inner_archive_ $i>](address: $crate::WrappedHeaderHash) -> ExternResult<$crate::WrappedHeaderHash> {
            delete_entry!(address.0.clone())?;
            // notify_goal_comment_archived(address.clone())?;
            Ok(address)
          }

          #[hdk_extern]
          pub fn [<archive_ $i>](address: $crate::WrappedHeaderHash) -> ExternResult<$crate::WrappedHeaderHash> {
            [<inner_archive_ $i>](address)
          }
        }
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
