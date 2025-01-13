//! Structures, traits, and methods that are meant for internal use.

mod index_range;
mod indexable_keywords;
mod normalize;
mod search;
mod search_and;
pub mod string_keywords;

// -----------------------------------------------------------------------------

pub mod search_top_scores;
pub use crate::simple::internal::search_top_scores::SearchTopScores;

// -----------------------------------------------------------------------------

#[cfg(any(feature = "eddie", feature = "rapidfuzz", feature = "strsim"))]
pub mod fuzzies;