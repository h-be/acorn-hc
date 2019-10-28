#![allow(unused_variables)]

#[macro_use] extern crate shrinkwraprs;
extern crate core;

#[derive(Shrinkwrap)]
pub struct Email(String);

#[test]
fn test_map_mut() {
  let mut email = Email("aoi.miyamori@musashino.jp".into());

  let len1 = email.map_ref(|s| s.len());
  email.map_mut(|s| s.push_str(".co"));
  let len2 = email.map_ref(|s| s.len());

  assert_eq!(len2, len1 + 3);
}
