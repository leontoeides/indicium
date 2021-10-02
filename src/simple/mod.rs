// Directories:
mod autocomplete;
mod internal;
mod search;

// Methods:
mod autocomplete_type;
mod builder;
mod default;
mod deref;
mod indexable;
mod indexable_keywords;
mod insert;
mod new;
mod remove;
mod replace;
mod search_index;
mod search_type;
mod string_keywords;
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