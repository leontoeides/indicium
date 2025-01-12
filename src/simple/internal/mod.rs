mod indexable_keywords;
mod search;
mod search_and;
pub mod search_top_scores;
pub mod string_keywords;

#[cfg(feature = "eddie")]
mod eddie;

#[cfg(feature = "rapidfuzz")]
mod rapidfuzz;

#[cfg(feature = "strsim")]
mod strsim;

#[cfg(any(feature = "eddie", feature = "rapidfuzz", feature = "strsim"))]
pub mod fuzzy_top_scores;

// -----------------------------------------------------------------------------

pub use crate::simple::internal::search_top_scores::SearchTopScores;

#[cfg(any(feature = "eddie", feature = "rapidfuzz", feature = "strsim"))]
pub use crate::simple::internal::fuzzy_top_scores::FuzzyTopScores;
