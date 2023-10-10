//! Used for tracking the top string similarity scores for (fuzzy matching) user
//! keywords that are not found in the search index.

mod find_bottom;
mod insert;
mod remove_bottom;
mod results;
mod with_capacity;

// -----------------------------------------------------------------------------

// Conditionally select hash map type based on feature flags:
#[cfg(feature = "ahash")]
use ahash::HashMap;
#[cfg(not(feature = "ahash"))]
use std::collections::HashMap;

// Static dependencies:
use kstring::KString;
use std::collections::BTreeSet;
use std::{cmp::Ord, cmp::PartialOrd, hash::Hash};

// -----------------------------------------------------------------------------
//
/// Tracks the top scoring keywords. This is intended to track the best _n_
/// matches for fuzzy string matching.

#[derive(Default)]
pub(crate) struct StrsimTopScores<'a, K: Hash + Ord, S: PartialOrd> {
    /// Tracks the top _n_ scores.
    pub(crate) top: HashMap<&'a KString, (&'a BTreeSet<K>, S)>,
    /// Tracks lowest of the top scores.
    pub(crate) bottom: Option<(&'a KString, S)>,
    /// Number of top scores to keep.
    pub(crate) capacity: usize,
} // StrsimTopScores