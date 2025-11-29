//! Contains interfaces to several string similarity and string distance crates.
//! These crates are used for autocompletion and fuzzy-matching user input.

// -----------------------------------------------------------------------------

#[cfg(feature = "eddie")]
mod eddie;

#[cfg(feature = "eddie")]
pub use crate::simple::internal::fuzzers::eddie::fuzzy::Eddie;

#[cfg(feature = "rapidfuzz")]
mod rapidfuzz;

#[cfg(feature = "rapidfuzz")]
pub use crate::simple::internal::fuzzers::rapidfuzz::fuzzy::Rapidfuzz;

#[cfg(feature = "strsim")]
mod strsim;

#[cfg(feature = "strsim")]
pub use crate::simple::internal::fuzzers::strsim::fuzzy::Strsim;

// -----------------------------------------------------------------------------
//
// Enumerations used for selecting the desired string similarity metric in the
// search index settings.

pub mod eddie_metric;
pub mod rapidfuzz_metric;
pub mod strsim_metric;

pub use crate::simple::internal::fuzzers::{
    rapidfuzz_metric::RapidfuzzMetric,
    eddie_metric::EddieMetric,
    strsim_metric::StrsimMetric
};

// -----------------------------------------------------------------------------
//
/// The `Fuzzy` trait allows `indicium` to treat the various string similarity
/// crates (such as `eddie`, `rapidfuzz`, `strsim`, etc.) generically.

#[cfg(any(feature = "strsim", feature = "eddie", feature = "rapidfuzz"))]
pub mod fuzzy;

#[cfg(any(feature = "strsim", feature = "eddie", feature = "rapidfuzz"))]
pub use crate::simple::internal::fuzzers::fuzzy::Fuzzy;

// -----------------------------------------------------------------------------
//
// Used for tracking the top string similarity scores for (fuzzy matching) user
// keywords that are not found in the search index.

#[cfg(any(feature = "strsim", feature = "eddie", feature = "rapidfuzz"))]
pub mod fuzzy_top_scores;

#[cfg(any(feature = "strsim", feature = "eddie", feature = "rapidfuzz"))]
pub use crate::simple::internal::fuzzers::fuzzy_top_scores::FuzzyTopScores;