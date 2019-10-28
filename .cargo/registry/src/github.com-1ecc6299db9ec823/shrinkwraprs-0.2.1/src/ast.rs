//! We want to make sure that the struct that our caller passes us is in the
//! right form. However, we don't want to clutter up our code generation
//! logic with lots of error handling. So instead, we take in our `DeriveInput`
//! and do all the error handling in one place, transforming it into an AST
//! specific to our crate if it's valid.

use syn;
use quote;

use itertools::Itertools;

type Fields = Vec<syn::Field>;

bitflags! {
  /// Controls which code and implementations we generate.
  pub struct ShrinkwrapFlags: u32 {
    const SW_MUT        = 0b00000001;
    const SW_IGNORE_VIS = 0b00000010;
  }
}

pub struct StructDetails {
  pub flags: ShrinkwrapFlags,
  pub ident: syn::Ident,
  pub generics: syn::Generics,
  pub visibility: syn::Visibility
}

/// Represents either a tuple or bracketed struct with at least one field.
pub struct Struct {
  pub inner_field: quote::Tokens,
  pub inner_type: syn::Type,
  pub inner_visibility: syn::Visibility
}

pub fn validate_derive_input(input: syn::DeriveInput) -> (StructDetails, Struct) {
  // Note that `unwrap()`s and `panic()`s are totally fine here; since we're
  // inside a procedural macro, panics happen at compile time

  use syn::{DeriveInput, DataStruct, FieldsUnnamed, FieldsNamed};
  use syn::Data::{Struct, Enum, Union};
  use syn::Fields::{Named, Unnamed};

  let DeriveInput { attrs, vis, ident, generics, data, .. } = input;

  let flags = shrinkwrap_flags(&attrs);
  let details = StructDetails { flags, ident, visibility: vis, generics };

  let input = match data {
    Struct(DataStruct { fields: Unnamed(FieldsUnnamed { unnamed: fields, .. }), .. }) => {
      let fields = fields.into_iter().collect_vec();
      validate_tuple(fields)
    },
    Struct(DataStruct { fields: Named(FieldsNamed { named: fields, .. }), .. }) => {
      let fields = fields.into_iter().collect_vec();
      validate_nontuple(fields)
    },
    Struct(..) =>
      panic!("shrinkwraprs needs a struct with at least one field!"),
    Enum(..) =>
      panic!("shrinkwraprs does not support enums"),
    Union(..) =>
      panic!("shrinkwraprs does not support C-style unions")
  };

  (details, input)
}

/// Specifically for working with attributes like #[shrinkwrap(..)], where
/// a name is combined with a list of attributes. Get the list of attributes
/// matching the tag.
fn tagged_attrs(tag: &str, attrs: &[syn::Attribute]) -> Vec<syn::NestedMeta> {
  use syn::{Meta, MetaList};

  let mut result = vec![];

  for attr in attrs {
    let meta = attr.interpret_meta();

    if let Some(Meta::List(MetaList { ident, nested, .. })) = meta {
      if &ident == tag {
        result.extend(nested);
      }
    }
  }

  result
}

fn shrinkwrap_flags(attrs: &[syn::Attribute]) -> ShrinkwrapFlags {
  use syn::{Meta, NestedMeta};

  let meta = tagged_attrs("shrinkwrap", attrs);
  let mut flags = ShrinkwrapFlags::empty();

  for attr in meta {
    if let NestedMeta::Meta(Meta::Word(ident)) = attr {
      if &ident == "mutable" {
        flags |= ShrinkwrapFlags::SW_MUT;
      } else if &ident == "unsafe_ignore_visibility" {
        flags |= ShrinkwrapFlags::SW_IGNORE_VIS;
      }
    }
  }

  flags
}

fn is_marked(field: &syn::Field) -> bool {
  use syn::{Meta, NestedMeta};

  let meta = tagged_attrs("shrinkwrap", &field.attrs);

  meta.into_iter().any(|meta| {
    if let NestedMeta::Meta(Meta::Word(ident)) = meta {
      &ident == "main_field"
    } else {
      false
    }
  })
}

/// Only a single field, out of all a struct's fields, can be marked as
/// the main field that we deref to. So let's find that field.
/// We also return the 0-based number of the marked field.
fn find_marked_field(fields: Fields) -> ((usize, syn::Field), Fields) {
  let (marked, unmarked) = fields.into_iter()
    .enumerate()
    .partition::<Vec<_>, _>(|&(_, ref field)| is_marked(field));
  let marked_len = marked.len();
  let single: Option<(_,)> = marked.into_iter()
    .collect_tuple();

  match (single, unmarked.len()) {
    (Some((field,)), _) => {
      let unmarked = unmarked.into_iter()
        .map(|(_, field)| field)
        .collect_vec();

      (field, unmarked)
    }
    (None, 1) => {
      let single: (_,) = unmarked.into_iter()
        .collect_tuple()
        .unwrap();

      (single.0, vec![])
    },
    _ => if marked_len == 0 {
      panic!("halp! shrinkwraprs doesn't know which field you want
this struct to convert to. Did you forget to mark a
field with #[shrinkwrap(main_field)]?");
    } else {
      panic!("halp! shrinkwraprs doesn't know which field you want
this struct to convert to. Did you accidentally mark
more than one field with #[shrinkwrap(main_field)]?");
    }
  }
}

fn validate_tuple(fields: Fields) -> Struct {
  if fields.len() == 0 {
    panic!("shrinkwraprs requires tuple structs to have at least one
field!");
  }

  let ((marked_index, marked_field), _) = find_marked_field(fields);
  let index: syn::Index = marked_index.into();
  let ty = marked_field.ty;
  let vis = marked_field.vis;

  Struct {
    inner_field: quote!( #index ),
    inner_type: ty,
    inner_visibility: vis
  }
}

fn validate_nontuple(fields: Fields) -> Struct {
  if fields.len() == 0 {
    panic!("shrinkwraprs requires structs to have at least one
field!");
  }

  let ((_, marked_field), _) = find_marked_field(fields);
  let ident = marked_field.ident
    .unwrap();
  let ty = marked_field.ty;
  let vis = marked_field.vis;

  Struct {
    inner_field: quote!( #ident ),
    inner_type: ty,
    inner_visibility: vis
  }
}

#[cfg(test)]
mod tests {
  use syn;
  use itertools::Itertools;

  use super::*;

  #[test]
  fn test_field_attribute_found() {
    let input = r"
      struct Foo {
        field1: u32,
        #[shrinkwrap(main_field)]
        field2: u32
      }
    ";

    let strct: syn::DeriveInput = syn::parse_str(input)
      .unwrap();

    match strct.data {
      syn::Data::Struct(syn::DataStruct { fields, .. }) => {
        let marked = fields.into_iter()
          .filter(|field| is_marked(field));
        let field: (&syn::Field,) = marked
          .collect_tuple()
          .unwrap();
        let ident = field.0.ident
          .unwrap();

        assert_eq!(&ident, "field2");
      },
      _ => panic!()
    }
  }

  #[test]
  fn test_field_attribute_not_found() {
    let input = r"
      struct Foo {
        field1: u32,
        field2: u32
      }
    ";

    let strct: syn::DeriveInput = syn::parse_str(input)
      .unwrap();

    match strct.data {
      syn::Data::Struct(syn::DataStruct { fields, .. }) => {
        let marked = fields.into_iter()
          .filter(|field| is_marked(field))
          .collect_vec();
        assert_eq!(marked.len(), 0);
      },
      _ => panic!()
    }
  }
}
