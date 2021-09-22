mod autocomplete;
mod conjunction;
mod default;
mod deref;
mod indexable;
mod indexable_keywords;
mod insert;
mod keyword_autocomplete;
mod keyword_search;
mod keyword_search_internal;
mod new;
mod remove;
mod replace;
mod search;
mod search_and;
mod search_index;
mod search_or;
mod string_keywords;
mod tests;

// -----------------------------------------------------------------------------

pub use crate::simple::indexable::Indexable;
pub use crate::simple::search_index::SearchIndex;