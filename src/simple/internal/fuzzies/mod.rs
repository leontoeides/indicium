//! Contains interfaces to several string similarity and string distance
//! algorithms. This algorithms are used for autocompletion and fuzzy-matching
//! user input.

// -----------------------------------------------------------------------------

#[cfg(feature = "eddie")]
mod eddie;

// #[cfg(feature = "eddie")]
// pub use crate::simple::internal::fuzzies::eddie::fuzzy::Eddie;

#[cfg(feature = "rapidfuzz")]
mod rapidfuzz;

// #[cfg(feature = "rapidfuzz")]
// pub use crate::simple::internal::fuzzies::rapidfuzz::fuzzy::Rapidfuzz;

#[cfg(feature = "strsim")]
mod strsim;

// #[cfg(feature = "strsim")]
// pub use crate::simple::internal::fuzzies::strsim::fuzzy::Strsim;

// -----------------------------------------------------------------------------
//
/// The `Fuzzy` trait allows `indicium` to treat the various string similarity
/// crates (such as `eddie`, `rapidfuzz`, `strsim`, etc.) generically.

// pub mod fuzzy;

// pub use crate::simple::internal::fuzzies::fuzzy::Fuzzy;

// -----------------------------------------------------------------------------
//
// Used for tracking the top string similarity scores for (fuzzy matching) user
// keywords that are not found in the search index.

pub mod top_scores;

pub use crate::simple::internal::fuzzies::top_scores::FuzzyTopScores;