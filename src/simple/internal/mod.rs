//! Structures, traits, and methods that are meant for internal use.

mod and_search;
mod index_range;
mod indexable_keywords;
mod keyword_search;
mod normalize;
pub mod string_keywords;

// -----------------------------------------------------------------------------

pub mod search_top_scores;
pub use crate::simple::internal::search_top_scores::SearchTopScores;

// -----------------------------------------------------------------------------

#[cfg(any(feature = "eddie", feature = "rapidfuzz", feature = "strsim"))]
pub mod fuzzies;