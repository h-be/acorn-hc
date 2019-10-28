//! Example usages of the mapping functions.

#![cfg(feature = "std")]
#![allow(unused_variables)]

#[macro_use] extern crate shrinkwraprs;

#[derive(Shrinkwrap)]
pub struct Email(String);

fn main() {
  let mut email = Email("aoi.miyamori@musashino.jp".into());

  let len = email.map_ref(|s| s.len());
  email.map_mut(|s| s.push_str(".co"));
  let s: String = email.map(|s| s);

  println!("done!");
}
