mod and;
mod internal;
mod keyword;
mod or;

mod autocomplete;
mod builder;
mod conjunction;
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
mod string_keywords;
mod tests;

// -----------------------------------------------------------------------------

pub use crate::simple::builder::SearchIndexBuilder;
pub use crate::simple::conjunction::Conjunction;
pub use crate::simple::indexable::Indexable;
pub use crate::simple::search_index::SearchIndex;