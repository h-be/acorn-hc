//! # shrinkwraprs
//!
//! Making wrapper types allows us to give more compile-time
//! guarantees about our code being correct:
//!
//! ```ignore
//! // Now we can't mix up widths and heights; the compiler will yell at us!
//! struct Width(u64);
//! struct Height(u64);
//! ```
//!
//! But... they're kind of a pain to work with. If you ever need to get at
//! that wrapped `u64`, you need to constantly pattern-match back and forth
//! to wrap and unwrap the values.
//!
//! `shrinkwraprs` aims to alleviate this pain by allowing you to derive
//! implementations of various conversion traits by deriving
//! `Shrinkwrap`.
//!
//! ## Functionality implemented
//!
//! Currently, using `#[derive(Shrinkwrap)]` will derive the following traits
//! for all structs:
//!
//! * `AsRef<InnerType>`
//! * `Borrow<InnerType>`
//! * `Deref<Target=InnerType>`
//!
//! It will also derive the following inherent methods:
//!
//! * `fn map<F, T>(self, mut f: F) -> T where F: FnMut(InnerType) -> T`
//! * `fn map_ref<F, T>(&self, mut f: F) -> T where F: FnMut(&InnerType) -> T`
//! * `fn map_mut<F, T>(&mut self, mut f: F) -> T where F: FnMut(&mut InnerType) -> T`
//!
//! `map_mut()` will have the same visibility as the inner field, which ensures
//! that `map_mut()` doesn't leak the possibility of changing the inner value
//! (potentially in invariant-violating ways). `map()` and `map_ref()` have the
//! same visibility as the struct itself, since these *don't* provide direct
//! ways for callers to break your data.
//!
//! Additionally, using `#[shrinkwrap(mutable)]` will also
//! derive the following traits:
//!
//! * `AsMut<InnerType>`
//! * `BorrowMut<InnerType>`
//! * `DerefMut<Target=InnerType>`
//!
//! ## Cool, how do I use it?
//!
//! ```ignore
//! #[macro_use] extern crate shrinkwraprs;
//!
//! #[derive(Shrinkwrap)]
//! struct Email(String);
//!
//! fn main() {
//!     let email = Email("chiya+snacks@natsumeya.jp".into());
//!
//!     let is_discriminated_email =
//!         email.contains("+");  // Woohoo, we can use the email like a string!
//!
//!     /* ... */
//! }
//! ```
//!
//! If you have multiple fields, but there's only one field you want to be able
//! to deref/borrow as, mark it with `#[shrinkwrap(main_field)]`:
//!
//! ```ignore
//! #[derive(Shrinkwrap)]
//! struct Email {
//!     spamminess: f64,
//!     #[shrinkwrap(main_field)] addr: String
//! }
//!
//! #[derive(Shrinkwrap)]
//! struct CodeSpan(u32, u32, #[shrinkwrap(main_field)] Token);
//! ```
//!
//! If you also want to be able to modify the wrapped value directly,
//! add the attribute `#[shrinkwrap(mutable)]` as well:
//!
//! ```ignore
//! #[derive(Shrinkwrap)]
//! #[shrinkwrap(mutable)]
//! struct InputBuffer {
//!     buffer: String
//! }
//!
//! ...
//! let mut input_buffer = /* ... */;
//! input_buffer.push_str("some values");
//! ...
//! ```

// Additionally, perhaps subsume some functionality from
// [`from_variants`](https://crates.io/crates/from_variants)?

#![cfg_attr(feature = "strict", deny(warnings))]
#![recursion_limit="128"]

extern crate proc_macro;
extern crate syn;
#[macro_use] extern crate quote;
extern crate itertools;
#[macro_use] extern crate bitflags;

use proc_macro::TokenStream;
use quote::{Tokens, ToTokens};

mod ast;
mod visibility;

#[proc_macro_derive(Shrinkwrap, attributes(shrinkwrap))]
pub fn shrinkwrap(tokens: TokenStream) -> TokenStream {
  use ast::{ShrinkwrapFlags, validate_derive_input};
  use visibility::field_visibility;
  use visibility::FieldVisibility::*;

  let input: syn::DeriveInput = syn::parse(tokens)
    .unwrap();
  let (details, input) = validate_derive_input(input);

  let mut tokens = Tokens::new();

  impl_immut_borrows(&details, &input)
    .to_tokens(&mut tokens);
  impl_map(&details, &input)
    .to_tokens(&mut tokens);

  if details.flags.contains(ShrinkwrapFlags::SW_MUT) {
    // Make sure that the inner field isn't less visible than the outer struct.
    if !details.flags.contains(ast::ShrinkwrapFlags::SW_IGNORE_VIS) {
      match field_visibility(&details.visibility, &input.inner_visibility) {
        Restricted =>
          panic!("shrinkwraprs: cowardly refusing to implement mutable
conversion traits because inner field is less visible
than shrinkwrapped struct. Implementing mutable traits
could allow violation of struct invariants. If you'd
like to override this, use
#[shrinkwrap(unsafe_ignore_visibility)] on your struct."),
        CantDetermine =>
          panic!("shrinkwraprs: cowardly refusing to implement mutable
conversion traits because I can't figure out whether
the inner field is as visible as the shrinkwrapped
struct or not. This is usually because there is a mix
of visibilities starting at the crate root and
visiblities starting at self/super. If you'd like to
override this, use #[shrinkwrap(unsafe_ignore_visibility)] on
your struct."),
        _ => ()
      }
    }

    impl_mut_borrows(&details, &input)
      .to_tokens(&mut tokens);
  }

  tokens.into()
}

// When generating our code, we need to be careful not to leak things into the
// surrounding code. For example, we don't use imports unless they're inside a
// scope, because otherwise we'd be inserting invisible imports whenever a user
// used #[derive(Shrinkwrap)].

fn impl_immut_borrows(details: &ast::StructDetails, input: &ast::Struct) -> Tokens {
  let &ast::StructDetails { ref ident, ref generics, .. } = details;
  let &ast::Struct { ref inner_field, ref inner_type, .. } = input;

  let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
  let rust = syn::Ident::from(RUST);

  quote! {
    impl #impl_generics ::#rust::ops::Deref for #ident #ty_generics #where_clause {
      type Target = #inner_type;
      fn deref(&self) -> &Self::Target {
        &self.#inner_field
      }
    }

    impl #impl_generics ::#rust::borrow::Borrow<#inner_type> for #ident #ty_generics #where_clause {
      fn borrow(&self) -> &#inner_type {
        &self.#inner_field
      }
    }

    impl #impl_generics ::#rust::convert::AsRef<#inner_type> for #ident #ty_generics #where_clause {
      fn as_ref(&self) -> &#inner_type {
        &self.#inner_field
      }
    }
  }
}

fn impl_mut_borrows(details: &ast::StructDetails, input: &ast::Struct) -> Tokens {
  let &ast::StructDetails { ref ident, ref generics, .. } = details;
  let &ast::Struct { ref inner_field, ref inner_type, .. } = input;

  let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
  let rust = syn::Ident::from(RUST);

  quote! {
    impl #impl_generics ::#rust::ops::DerefMut for #ident #ty_generics #where_clause {
      fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.#inner_field
      }
    }

    impl #impl_generics ::#rust::borrow::BorrowMut<#inner_type> for #ident #ty_generics #where_clause {
      fn borrow_mut(&mut self) -> &mut #inner_type {
        &mut self.#inner_field
      }
    }

    impl #impl_generics ::#rust::convert::AsMut<#inner_type> for #ident #ty_generics #where_clause {
      fn as_mut(&mut self) -> &mut #inner_type {
        &mut self.#inner_field
      }
    }
  }
}

fn impl_map(details: &ast::StructDetails, input: &ast::Struct) -> Tokens {
  let &ast::StructDetails { ref ident, ref generics, .. } = details;
  let &ast::Struct { ref inner_field, ref inner_type, ref inner_visibility } = input;

  let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

  // This is a *massive* hack to avoid variable capture, but I can't figure out
  // how to get `quote` to enforce hygiene or generate a gensym.
  let f = quote!( __SHRINKWRAP_F );
  let t = quote!( __SHRINKWRAP_T );

  quote! {
    #[allow(dead_code, non_camel_case_types)]
    impl #impl_generics #ident #ty_generics #where_clause {
      /// Map a function over the wrapped value, consuming it in the process.
      pub fn map<#t, #f: FnMut(#inner_type) -> #t>(self, mut f: #f) -> #t {
        f(self.#inner_field)
      }

      /// Map a function over the wrapped value without consuming it.
      pub fn map_ref<#t, #f: FnMut(&#inner_type) -> #t>(&self, mut f: #f) -> #t {
        f(&self.#inner_field)
      }

      /// Map a function over the wrapped value, potentially changing it in place.
      #inner_visibility fn map_mut<#t, #f>(&mut self, mut f: #f) -> #t
        where #f: FnMut(&mut #inner_type) -> #t
      {
        f(&mut self.#inner_field)
      }
    }
  }
}

#[cfg(feature = "std")]
const RUST: &str = "std";
#[cfg(not(feature = "std"))]
const RUST: &str = "core";
