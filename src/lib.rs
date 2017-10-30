// -*- mode: rust; -*-

//! This library implements a logical resolution solver for boolean logic. 
//! To solve a clause you need to provide it in conjunctive-normal-form. Conversion
//! isn't currently supported.
//!
//! Generally: this crate is early-early in development and also kind of my way of 
//! learning the best practises of Rust development. I have some cool ideas for 
//! the 1.0-milestone as well as additional features that could be added post-1.0 to 
//! make lrs into a general purpose logic processing library.
//!
//! To chat about this or any of my other projects, get in touch with me?
//!
//! Katharina Fey
//!     https://twitter.com/spacekookie
//!     https://spacekookie.de
//!     <kookie (at) spacekookie (dot) de)

#[macro_use] mod terms;
pub use terms::*;

#[macro_use] mod clause;
pub use clause::*;

#[macro_use] mod result;
pub use result::*;

/* Unit tests below */
mod tests;
