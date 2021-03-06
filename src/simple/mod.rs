//! The simple Indicium search implementation. Fewer bells-and-whistles but
//! easier to use than the other options.
//!
//! There will be more search implementations in future versions.

// Directories:
mod autocomplete;
mod internal;
mod search;

// Methods & structs:
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
mod strsim_type;
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
pub use crate::simple::strsim_type::StrSimType;