//! The simple Indicium search implementation. Fewer bells-and-whistles but
//! easier to use than the other options.
//!
//! There might be more search implementations in future versions.

#[cfg(all(feature = "eddie", feature = "rapidfuzz"))]
compile_error!("features `eddie` and `rapidfuzz` cannot both be enabled");

#[cfg(all(feature = "eddie", feature = "strsim"))]
compile_error!("features `eddie` and `strsim` cannot both be enabled");

#[cfg(all(feature = "rapidfuzz", feature = "strsim"))]
compile_error!("features `rapidfuzz` and `strsim` cannot both be enabled");

#[cfg(all(feature = "ahash", feature = "gxhash"))]
compile_error!("features `ahash` and `gxhash` cannot both be enabled");

#[cfg(all(feature = "ahash", feature = "rustc-hash"))]
compile_error!("features `ahash` and `rustc-hash` cannot both be enabled");

#[cfg(all(feature = "gxhash", feature = "rustc-hash"))]
compile_error!("features `gxhash` and `rustc-hash` cannot both be enabled");

// Directories:
mod autocomplete;
mod internal;
mod search;

// Methods, structs & implementations:
mod autocomplete_type;
mod builder;
mod clear;
mod default;
mod deref;
mod deref_mut;
mod dump_keyword;
mod indexable;
mod insert;
mod max_keys_per_keyword;
mod new;
mod remove;
mod replace;
mod search_index;
mod search_type;
mod tests;

// For debug builds only:
#[cfg(debug_assertions)]
mod profile;

// -----------------------------------------------------------------------------

pub use crate::simple::autocomplete_type::AutocompleteType;
pub use crate::simple::builder::SearchIndexBuilder;
pub use crate::simple::indexable::Indexable;
pub use crate::simple::search_index::SearchIndex;
pub use crate::simple::search_type::SearchType;

pub use crate::simple::internal::fuzzers::{
    RapidfuzzMetric,
    EddieMetric,
    StrsimMetric
};