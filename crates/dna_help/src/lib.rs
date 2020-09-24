use hdk3::prelude::*;

pub type EntryAndHash<T> = (T, HeaderHash);
pub type OptionEntryAndHash<T> = Option<EntryAndHash<T>>;

pub fn get_latest_for_entry<T: TryFrom<SerializedBytes, Error = SerializedBytesError>>(
    entry_address: EntryHash,
) -> ExternResult<OptionEntryAndHash<T>> {
    // First, make sure we DO have the latest entry address
    let maybe_latest_entry_address = match get_details!(entry_address.clone())? {
        Some(Details::Entry(details)) => match details.updates.len() {
            0 => Some(entry_address),
            _ => {
                let mut sortlist = details.updates.to_vec();
                // unix timestamp should work for sorting
                sortlist.sort_by_key(|header| header.timestamp.0);
                // sorts in ascending order, so take the last element
                Some(sortlist.last().unwrap().entry_hash.clone())
            }
        },
        _ => None,
    };

    // Second, go and get that entry, and return it and its header_address
    match maybe_latest_entry_address {
        Some(latest_entry_address) => match get!(latest_entry_address)? {
            Some(element) => match element.entry().to_app_option::<T>()? {
                Some(entry) => Ok(Some((entry, element.header_address().to_owned()))),
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
        use paste::paste;

        paste! {
          pub const [<$i:upper _PATH>]: &str = $path;

          #[derive(Serialize, Deserialize, Debug, Clone, PartialEq, SerializedBytes)]
          pub struct [<$crud_type WireEntry>] {
              pub entry: $crud_type,
              pub address: HeaderHash,
          }

          impl From<$crate::EntryAndHash<$crud_type>> for [<$crud_type WireEntry>] {
              fn from(entry_and_hash: $crate::EntryAndHash<$crud_type>) -> Self {
                [<$crud_type WireEntry>] {
                      entry: entry_and_hash.0,
                      address: entry_and_hash.1,
                  }
              }
          }

          #[derive(Serialize, Deserialize, Debug, Clone, PartialEq, SerializedBytes)]
          pub struct [<Vec $crud_type WireEntry>](Vec<[<$crud_type WireEntry>]>);

          /*
            CREATE
          */
          pub fn [<inner_create_ $i>](entry: $crud_type) -> ExternResult<[<$crud_type WireEntry>]> {
              let address = create_entry!(entry.clone())?;
              let entry_hash = hash_entry!(entry.clone())?;
              let path_hash = Path::from([<$i:upper _PATH>]).hash()?;
              create_link!(path_hash, entry_hash)?;
              let wire_entry = [<$crud_type WireEntry>] { entry, address };
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
          pub fn [<inner_update_ $i>](update: [<$crud_type WireEntry>]) -> ExternResult<[<$crud_type WireEntry>]> {
            update_entry!(update.address.clone(), update.entry.clone())?;
            let wire_entry = [<$crud_type WireEntry>] {
                entry: update.entry,
                address: update.address,
            };
            // notify_goal_comment(wire_entry.clone())?;
            Ok(wire_entry)
          }

          #[hdk_extern]
          pub fn [<update_ $i>](update: [<$crud_type WireEntry>]) -> ExternResult<[<$crud_type WireEntry>]> {
            [<inner_update_ $i>](update)
          }

          /*
            DELETE
          */
          pub fn [<inner_archive_ $i>](address: HeaderHash) -> ExternResult<HeaderHash> {
            delete_entry!(address.clone())?;
              // notify_goal_comment_archived(address.clone())?;
              Ok(address)
          }

          #[hdk_extern]
          pub fn [<archive_ $i>](address: HeaderHash) -> ExternResult<HeaderHash> {
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
