use hdk3::prelude::*;

pub fn get_latest_for_entry<T: TryFrom<SerializedBytes>>(
  entry_address: EntryHash,
) -> ExternResult<Option<(T, HeaderHash)>> {
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
          Some(element) => match element.entry().to_app_option::<T>() {
              Ok(option) => match option {
                  Some(entry) => Ok(Some((entry, element.header_address().to_owned()))),
                  None => Ok(None),
              },
              Err(_) => Err(HdkError::SerializedBytes(SerializedBytesError::FromBytes(
                  "failed to deserialize".into()
              ))),
          },
          None => Ok(None),
      },
      None => Ok(None),
  }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
