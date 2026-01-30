//! A collection of convenience traits

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "path_to_string")]
mod path;
#[cfg(feature = "path_to_string")]
pub use path::*;

#[cfg(feature = "inspect_none")]
mod inspect_none;
#[cfg(feature = "inspect_none")]
pub use inspect_none::*;

#[cfg(feature = "discard")]
mod discard;
#[cfg(feature = "discard")]
pub use discard::*;

#[cfg(feature = "permit")]
mod permit;
#[cfg(feature = "permit")]
pub use permit::*;

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;
