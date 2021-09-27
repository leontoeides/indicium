mod and;
mod internal;
mod keyword;
mod or;

mod autocomplete;
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
mod search;
mod search_index;
mod search_type;
mod string_keywords;
mod tests;

// -----------------------------------------------------------------------------

pub use crate::simple::autocomplete_type::AutocompleteType;
pub use crate::simple::builder::SearchIndexBuilder;
pub use crate::simple::indexable::Indexable;
pub use crate::simple::search_index::SearchIndex;
pub use crate::simple::search_type::SearchType;