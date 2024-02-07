mod indexable_keywords;
mod search;
mod search_and;
pub mod search_top_scores;
pub mod string_keywords;

#[cfg(feature = "strsim")]
mod strsim;

#[cfg(feature = "eddie")]
mod eddie;

#[cfg(any(feature = "strsim", feature = "eddie"))]
pub mod fuzzy_top_scores;

// -----------------------------------------------------------------------------

pub use crate::simple::internal::search_top_scores::SearchTopScores;

#[cfg(any(feature = "strsim", feature = "eddie"))]
pub use crate::simple::internal::fuzzy_top_scores::FuzzyTopScores;